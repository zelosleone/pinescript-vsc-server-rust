use std::collections::{HashMap, HashSet};

use tower_lsp::lsp_types::{DiagnosticSeverity, DocumentSymbol, Position, Range};
use tree_sitter::Node;

use crate::analysis::{Analyzer, Scope, SymbolDef, SymbolKind, SymbolRef};
use crate::types::{BaseType, Type};

impl<'a> Analyzer<'a> {
    pub(super) fn seed_builtins(&mut self) {
        let value_list: Vec<(String, Type)> = self
            .builtins
            .values()
            .map(|(name, ty)| (name.clone(), ty.clone()))
            .collect();
        for (name, ty) in value_list {
            let range = Range::default();
            self.define_symbol(name, SymbolKind::Builtin, range, range, ty, true);
        }
    }

    // Scans the raw text for `enum` declarations and registers the enum type
    // and its members into the analyzer state. This is a pragmatic fallback
    // until the tree-sitter grammar includes enum_declaration nodes.
    pub(super) fn collect_enum_declarations(&mut self) {
        let lines: Vec<&str> = self.text.lines().collect();
        let mut line_offsets = Vec::with_capacity(lines.len());
        for idx in 0..lines.len() {
            line_offsets.push(
                self.line_index
                    .position_to_offset(self.text, Position::new(idx as u32, 0)),
            );
        }
        let make_range = |start: usize, len: usize| Range {
            start: self.line_index.offset_to_position(self.text, start),
            end: self.line_index.offset_to_position(self.text, start + len),
        };

        for (i, line) in lines.iter().enumerate() {
            let trimmed_start = line.trim_start();
            let mut rest = trimmed_start;
            if rest.starts_with("export ") {
                rest = rest.trim_start_matches("export ").trim_start();
            }
            if let Some(stripped) = rest.strip_prefix("enum ") {
                let after = stripped.trim();
                if after.is_empty() {
                    continue;
                }
                let name = after.split_whitespace().next().unwrap();

                // compute byte offset for the start of the name in the document
                let line_start_offset = line_offsets.get(i).copied().unwrap_or(0);
                // find name position in the original line (not trimmed)
                let pos_in_line = line.find(name).unwrap_or(0);
                let name_start = line_start_offset + pos_in_line;
                let selection_range = make_range(name_start, name.len());
                let range = selection_range;

                // Register type symbol for enum
                self.define_symbol(
                    name.to_string(),
                    SymbolKind::Type,
                    range,
                    selection_range,
                    Type::UserDefined(name.to_string()),
                    false,
                );

                // Collect member identifiers in subsequent indented lines
                let mut members: Vec<String> = Vec::new();
                let mut member_ranges: std::collections::HashMap<String, Range> =
                    std::collections::HashMap::new();
                let mut j = i + 1;
                while j < lines.len() {
                    let next_line = lines[j];
                    if next_line.trim().is_empty() {
                        j += 1;
                        continue;
                    }
                    if next_line
                        .chars()
                        .next()
                        .map(|c| c == ' ' || c == '\t')
                        .unwrap_or(false)
                    {
                        let trimmed = next_line.trim();
                        let mut parts = trimmed.split('=');
                        let member = parts
                            .next()
                            .unwrap_or("")
                            .split_whitespace()
                            .next()
                            .unwrap_or("");
                        let title = parts.next().map(|value| value.trim());
                        if !member.is_empty() {
                            let line_start_offset = line_offsets.get(j).copied().unwrap_or(0);
                            let member_pos = next_line.find(member).unwrap_or(0);
                            let member_start = line_start_offset + member_pos;
                            let member_range = make_range(member_start, member.len());

                            if let Some(previous_range) = member_ranges.get(member) {
                                self.push_diagnostic(
                                    member_range,
                                    DiagnosticSeverity::ERROR,
                                    format!(
                                        "Duplicate enum member `{}` in enum `{}`",
                                        member, name
                                    ),
                                    "validation".to_string(),
                                );
                                self.push_diagnostic(
                                    *previous_range,
                                    DiagnosticSeverity::INFORMATION,
                                    format!("Previous enum member `{}` defined here", member),
                                    "validation".to_string(),
                                );
                            } else {
                                member_ranges.insert(member.to_string(), member_range);
                                members.push(member.to_string());
                            }

                            if let Some(title) = title {
                                let title_slice = title.split("//").next().unwrap_or("").trim_end();
                                let is_string = title_slice.len() >= 2
                                    && ((title_slice.starts_with('"')
                                        && title_slice.ends_with('"'))
                                        || (title_slice.starts_with('\'')
                                            && title_slice.ends_with('\'')));
                                if (title_slice.is_empty() || !is_string)
                                    && let Some(eq_idx) = next_line.find('=')
                                {
                                    let raw_after_eq = &next_line[eq_idx + 1..];
                                    let ws_prefix_len =
                                        raw_after_eq.len() - raw_after_eq.trim_start().len();
                                    let title_start_in_line = eq_idx + 1 + ws_prefix_len;
                                    let title_len = title_slice.len();
                                    let title_start = line_start_offset + title_start_in_line;
                                    let range = make_range(title_start, title_len.max(1));
                                    self.push_diagnostic(
                                        range,
                                        DiagnosticSeverity::ERROR,
                                        format!("Invalid enum title for member `{}`", member),
                                        "validation".to_string(),
                                    );
                                }
                            }
                        }
                        j += 1;
                    } else {
                        break;
                    }
                }
                if !members.is_empty() {
                    self.enums.insert(name.to_string(), members);
                    self.enum_member_ranges
                        .insert(name.to_string(), member_ranges);
                }
            }
        }
    }

    pub(super) fn handle_function_declaration(&mut self, node: Node) {
        let name_node = node
            .child_by_field_name("function")
            .or_else(|| node.child_by_field_name("method"));
        let Some(name_node) = name_node else {
            return;
        };
        let name = self.node_text(name_node);
        let range = self.range_for_node(name_node);
        let def_index = self.define_symbol(
            name.clone(),
            SymbolKind::Function,
            self.range_for_node(node),
            range,
            Type::unknown(),
            false,
        );
        self.function_calls.entry(name.clone()).or_default();

        self.enter_scope();

        self.function_stack.push(name);
        for i in 0..node.child_count() {
            if let Some(child) = node.child(i as u32)
                && node.field_name_for_child(i as u32) == Some("argument")
            {
                let arg_name = self.node_text(child);
                let arg_range = self.range_for_node(child);
                self.define_symbol(
                    arg_name,
                    SymbolKind::Parameter,
                    arg_range,
                    arg_range,
                    Type::unknown(),
                    false,
                );
            }
        }

        for i in 0..node.child_count() {
            if let Some(child) = node.child(i as u32) {
                match node.field_name_for_child(i as u32) {
                    Some("default_value") | Some("body") => self.walk(child),
                    _ => {}
                }
            }
        }
        self.function_stack.pop();

        self.exit_scope();

        let body_type = self.infer_return_type(node);
        if let Some(def) = self.definitions.get_mut(def_index)
            && body_type != Type::Unknown
        {
            def.ty = body_type;
        }
    }

    pub(super) fn handle_variable_definition(&mut self, node: Node) {
        let Some(var_node) = node.child_by_field_name("variable") else {
            return;
        };
        let name = self.node_text(var_node);
        let selection_range = self.range_for_node(var_node);
        let range = self.range_for_node(node);

        let declared_type = self.parse_declared_type(node);

        let init_node = node
            .child_by_field_name("initial_value")
            .or_else(|| node.child_by_field_name("initial_structure"));
        let inferred_type = init_node
            .map(|expr| self.infer_expr_type(expr))
            .unwrap_or_else(Type::unknown);
        let final_type = if declared_type != Type::Unknown {
            declared_type
        } else {
            inferred_type.clone()
        };

        self.define_symbol(
            name.clone(),
            SymbolKind::Variable,
            range,
            selection_range,
            final_type,
            false,
        );

        if let Some(expr) = init_node {
            self.check_duplicate_expression(expr, selection_range);
            self.update_collection_sizes(&name, expr);
            self.record_draw_object_assignment(&name, expr, selection_range);
            self.walk(expr);
        }
    }

    pub(super) fn handle_tuple_declaration(&mut self, node: Node) {
        let init_node = node
            .child_by_field_name("initial_value")
            .or_else(|| node.child_by_field_name("initial_structure"));
        let tuple_type = init_node
            .map(|expr| self.infer_expr_type(expr))
            .unwrap_or_else(Type::unknown);

        let mut tuple_items = Vec::new();
        if let Type::Tuple(items) = tuple_type {
            tuple_items = items;
        }

        let mut var_nodes = Vec::new();
        for i in 0..node.child_count() {
            if let Some(child) = node.child(i as u32)
                && node.field_name_for_child(i as u32) == Some("variables")
                && child.kind() == "identifier"
            {
                var_nodes.push(child);
            }
        }

        for (idx, var_node) in var_nodes.iter().enumerate() {
            let name = self.node_text(*var_node);
            let selection_range = self.range_for_node(*var_node);
            let ty = tuple_items.get(idx).cloned().unwrap_or_else(Type::unknown);
            self.define_symbol(
                name,
                SymbolKind::Variable,
                self.range_for_node(*var_node),
                selection_range,
                ty,
                false,
            );
        }
    }

    pub(super) fn handle_reassignment(&mut self, node: Node) {
        let Some(var_node) = node.child_by_field_name("variable") else {
            return;
        };
        if var_node.kind() == "identifier" {
            let name = self.node_text(var_node);
            let range = self.range_for_node(var_node);
            let def_index = self.resolve_symbol(&name);
            if def_index.is_none() {
                self.push_diagnostic(
                    range,
                    DiagnosticSeverity::ERROR,
                    format!("Reassignment to undefined variable `{}`", name),
                    "runtime".to_string(),
                );
            }
            self.references.push(SymbolRef {
                name,
                range,
                def_index,
            });
            let value_node = node
                .child_by_field_name("value")
                .or_else(|| node.child_by_field_name("structure"));
            if let Some(value_node) = value_node {
                let selection_range = self.range_for_node(var_node);
                self.record_draw_object_assignment(
                    &self.node_text(var_node),
                    value_node,
                    selection_range,
                );
            }
        }
    }

    pub(super) fn handle_for_statement(&mut self, node: Node) {
        self.check_for_statement_bounds(node);
        if let Some(counter) = node.child_by_field_name("counter") {
            let name = self.node_text(counter);
            let selection_range = self.range_for_node(counter);
            self.define_symbol(
                name,
                SymbolKind::LoopVariable,
                selection_range,
                selection_range,
                Type::scalar(BaseType::Int),
                false,
            );
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.walk(child);
        }
    }

    pub(super) fn handle_identifier_reference(&mut self, node: Node) {
        let Some(parent) = node.parent() else {
            return;
        };
        if self.is_definition_identifier(node, parent) {
            return;
        }

        if parent.kind() == "call"
            && parent
                .child_by_field_name("function")
                .map(|child| child.id() == node.id())
                .unwrap_or(false)
        {
            return;
        }

        if parent.kind() == "attribute"
            && parent
                .child_by_field_name("attribute")
                .map(|child| child.id() == node.id())
                .unwrap_or(false)
        {
            return;
        }

        let name = self.node_text(node);
        if name == "true" || name == "false" {
            return;
        }
        if self.is_builtin_namespace(&name) {
            return;
        }

        let range = self.range_for_node(node);
        let def_index = self.resolve_symbol(&name);
        if def_index.is_none() {
            self.push_diagnostic(
                range,
                DiagnosticSeverity::ERROR,
                format!("Unknown identifier `{}`", name),
                "validation".to_string(),
            );
        }
        self.references.push(SymbolRef {
            name,
            range,
            def_index,
        });
    }

    pub(super) fn is_definition_identifier(&self, node: Node, parent: Node) -> bool {
        match parent.kind() {
            "variable_definition" | "variable_definition_statement" => parent
                .child_by_field_name("variable")
                .map(|child| child.id() == node.id())
                .unwrap_or(false),
            "function_declaration_statement" => parent
                .child_by_field_name("function")
                .or_else(|| parent.child_by_field_name("method"))
                .map(|child| child.id() == node.id())
                .unwrap_or(false),
            "for_statement" => parent
                .child_by_field_name("counter")
                .map(|child| child.id() == node.id())
                .unwrap_or(false),
            "tuple_declaration" | "tuple_declaration_statement" => parent
                .child_by_field_name("variables")
                .map(|child| child.id() == node.id())
                .unwrap_or(false),
            "keyword_argument" => parent
                .child_by_field_name("key")
                .map(|child| child.id() == node.id())
                .unwrap_or(false),
            _ => false,
        }
    }

    pub(super) fn define_symbol(
        &mut self,
        name: String,
        kind: SymbolKind,
        range: Range,
        selection_range: Range,
        ty: Type,
        is_builtin: bool,
    ) -> usize {
        if let Some(scope) = self.scopes.last()
            && let Some(existing) = scope.symbols.get(&name)
        {
            let existing_range = self.definitions[*existing].selection_range;
            self.push_diagnostic(
                selection_range,
                DiagnosticSeverity::WARNING,
                format!("Duplicate definition of `{}`", name),
                "style".to_string(),
            );
            self.push_diagnostic(
                existing_range,
                DiagnosticSeverity::INFORMATION,
                format!("Previous definition of `{}`", name),
                "style".to_string(),
            );
        }

        if !is_builtin && self.settings.enable_shadowing_warnings {
            let current_depth = self.scopes.len().saturating_sub(1);
            if let Some(existing) = self.resolve_symbol(&name) {
                let (existing_depth, existing_range) = {
                    let existing_def = &self.definitions[existing];
                    (existing_def.scope_depth, existing_def.selection_range)
                };
                if existing_depth < current_depth {
                    self.push_diagnostic(
                        selection_range,
                        DiagnosticSeverity::HINT,
                        format!("`{}` shadows an outer definition", name),
                        "style".to_string(),
                    );
                    self.push_diagnostic(
                        existing_range,
                        DiagnosticSeverity::INFORMATION,
                        format!("`{}` previously defined here", name),
                        "style".to_string(),
                    );
                }
            }
        }

        let def_index = self.definitions.len();
        self.definitions.push(SymbolDef {
            name: name.clone(),
            kind,
            range,
            selection_range,
            ty,
            scope_depth: self.scopes.len().saturating_sub(1),
            is_builtin,
        });

        if let Some(scope) = self.scopes.last_mut() {
            scope.symbols.insert(name, def_index);
        }

        def_index
    }

    pub(super) fn resolve_symbol(&self, name: &str) -> Option<usize> {
        for scope in self.scopes.iter().rev() {
            if let Some(index) = scope.symbols.get(name) {
                return Some(*index);
            }
        }
        None
    }

    pub(super) fn enter_scope(&mut self) {
        self.scopes.push(Scope {
            symbols: HashMap::new(),
        });
    }

    pub(super) fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    pub(super) fn report_unused_symbols(&mut self) {
        let mut referenced = HashSet::new();
        for reference in &self.references {
            if let Some(def_index) = reference.def_index {
                referenced.insert(def_index);
            }
        }

        let mut unused = Vec::new();
        for (index, def) in self.definitions.iter().enumerate() {
            if def.is_builtin {
                continue;
            }
            if referenced.contains(&index) {
                continue;
            }
            unused.push((def.selection_range, def.kind_name(), def.name.clone()));
        }

        for (range, kind_name, name) in unused {
            self.push_diagnostic(
                range,
                DiagnosticSeverity::HINT,
                format!("Unused {} `{}`", kind_name, name),
                "style".to_string(),
            );
        }

        let mut used_enum_members = HashSet::new();
        for reference in &self.references {
            let mut parts = reference.name.splitn(2, '.');
            let enum_name = match parts.next() {
                Some(name) => name,
                None => continue,
            };
            let rest = match parts.next() {
                Some(rest) => rest,
                None => continue,
            };
            if !self.enums.contains_key(enum_name) {
                continue;
            }
            let member = rest.split('.').next().unwrap_or(rest);
            used_enum_members.insert(format!("{}.{}", enum_name, member));
        }

        let mut unused_enum_members = Vec::new();
        for (enum_name, members) in &self.enums {
            let Some(member_ranges) = self.enum_member_ranges.get(enum_name) else {
                continue;
            };
            for member in members {
                let key = format!("{}.{}", enum_name, member);
                if used_enum_members.contains(&key) {
                    continue;
                }
                let Some(range) = member_ranges.get(member) else {
                    continue;
                };
                unused_enum_members.push((*range, key));
            }
        }

        for (range, key) in unused_enum_members {
            self.push_diagnostic(
                range,
                DiagnosticSeverity::HINT,
                format!("Unused enum member `{}`", key),
                "style".to_string(),
            );
        }
    }

    pub(super) fn report_use_before_definition(&mut self) {
        let mut seen = HashSet::new();
        let mut hints = Vec::new();

        for reference in &self.references {
            if reference.def_index.is_some() {
                continue;
            }
            if !seen.insert(reference.name.clone()) {
                continue;
            }
            let ref_pos = reference.range.start;
            let mut has_later_def = None;
            for def in &self.definitions {
                if def.is_builtin || def.name != reference.name {
                    continue;
                }
                let def_pos = def.selection_range.start;
                let is_after = def_pos.line > ref_pos.line
                    || (def_pos.line == ref_pos.line && def_pos.character > ref_pos.character);
                if is_after {
                    has_later_def = Some(def.selection_range);
                    break;
                }
            }
            if let Some(def_range) = has_later_def {
                hints.push((reference.range, reference.name.clone(), def_range));
            }
        }

        for (range, name, def_range) in hints {
            self.push_diagnostic(
                range,
                DiagnosticSeverity::HINT,
                format!(
                    "Possible use before definition of `{}`; definition appears later",
                    name
                ),
                "logic".to_string(),
            );
            self.push_diagnostic(
                def_range,
                DiagnosticSeverity::INFORMATION,
                format!("`{}` defined here", name),
                "logic".to_string(),
            );
        }
    }

    #[allow(deprecated)]
    pub(super) fn build_document_symbols(&mut self) {
        for def in &self.definitions {
            if def.is_builtin {
                continue;
            }
            if def.scope_depth > 0 {
                continue;
            }
            self.document_symbols.push(DocumentSymbol {
                name: def.name.clone(),
                detail: Some(def.ty.display_name()),
                kind: def.kind.to_lsp(),
                tags: None,
                deprecated: None,
                range: def.range,
                selection_range: def.selection_range,
                children: None,
            });
        }
    }
}
