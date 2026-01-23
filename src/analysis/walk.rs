use tower_lsp::lsp_types::DiagnosticSeverity;
use tree_sitter::Node;

use crate::analysis::{Analyzer, ConditionContext, SymbolKind, SymbolRef};
use crate::types::Type;

impl<'a> Analyzer<'a> {
    pub(super) fn collect_syntax_errors(&mut self, node: Node) {
        if node.is_error() || node.is_missing() || node.kind() == "ERROR" {
            let range = self.range_for_node(node);
            self.push_diagnostic(
                range,
                DiagnosticSeverity::ERROR,
                "Syntax error".to_string(),
                "syntax".to_string(),
            );
        }
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.collect_syntax_errors(child);
        }
    }

    pub(super) fn walk(&mut self, node: Node) {
        match node.kind() {
            "function_declaration_statement" => {
                self.handle_function_declaration(node);
                return;
            }
            "variable_definition" | "variable_definition_statement" => {
                self.handle_variable_definition(node);
                return;
            }
            "tuple_declaration" | "tuple_declaration_statement" => {
                self.handle_tuple_declaration(node);
                return;
            }
            "reassignment" | "reassignment_statement" => {
                self.handle_reassignment(node);
                return;
            }
            "for_statement" => {
                self.handle_for_statement(node);
                return;
            }
            "while_statement" => {
                self.handle_while_statement(node);
                return;
            }
            "if_statement" => {
                self.handle_if_statement(node);
                return;
            }
            "call" => {
                self.handle_call(node);
                return;
            }
            "enum_declaration" => {
                // Enum declarations are collected from text for now; skip inner identifiers.
                return;
            }
            "block" => {
                self.enter_scope();
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    self.walk(child);
                }
                self.exit_scope();
                return;
            }
            "identifier" => {
                self.handle_identifier_reference(node);
            }
            "attribute" => {
                // Record attribute references (e.g., EnumName.member)
                if let Some(name) = self.attribute_chain_name(node) {
                    if name == "barstate.isconfirmed" {
                        self.has_barstate_isconfirmed = true;
                    }
                    let range = self.range_for_node(node);
                    let def_index = name
                        .split('.')
                        .next()
                        .and_then(|obj| self.resolve_symbol(obj));
                    self.references.push(SymbolRef {
                        name,
                        range,
                        def_index,
                    });
                    return;
                }
            }
            "math_operation" => {
                self.check_math_operation(node);
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    self.walk(child);
                }
                return;
            }
            "subscript" => {
                self.check_history_reference(node);
                let mut cursor = node.walk();
                for child in node.children(&mut cursor) {
                    self.walk(child);
                }
                return;
            }
            _ => {}
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.walk(child);
        }
    }

    pub(super) fn handle_call(&mut self, node: Node) {
        let Some(function_node) = node.child_by_field_name("function") else {
            return;
        };
        let call_name = self.call_name_from_node(function_node);

        if call_name == "indicator" || call_name == "strategy" {
            self.has_indicator_or_strategy = true;
        }
        if call_name == "indicator" {
            self.has_indicator_declaration = true;
        }
        if call_name == "strategy" {
            self.has_strategy_declaration = true;
        }

        let range = self.range_for_node(function_node);
        let def_index = self.resolve_symbol(&call_name);
        if def_index.is_none() && !self.builtins.is_function(&call_name) {
            self.push_diagnostic(
                range,
                DiagnosticSeverity::ERROR,
                format!("Unknown function `{}`", call_name),
                "validation".to_string(),
            );
        }
        self.references.push(SymbolRef {
            name: call_name.clone(),
            range,
            def_index,
        });

        let raw_args = Self::collect_call_arguments(node);
        let args: Vec<(Option<String>, Node)> = raw_args
            .into_iter()
            .map(|(opt_name_node, value)| (opt_name_node.map(|n| self.node_text(n)), value))
            .collect();
        self.update_max_bars_back(&call_name, &args);
        self.update_strategy_settings(&call_name, &args, range);
        self.check_request_security_lookahead(&call_name, &args, range);
        self.check_request_security_gaps(&call_name, &args, range);
        self.check_request_security_format(&call_name, &args, range);
        self.check_request_security_expression(&call_name, &args, range);
        self.check_input_range(&call_name, &args);
        self.check_strategy_call_logic(&call_name, &args, range);
        self.record_strategy_order_id(&call_name, &args, range);
        self.check_strategy_direction_conflict(&call_name, &args, range);
        self.check_strategy_oca_conflicts(&call_name, &args);
        self.check_strategy_quantity(&call_name, &args);
        self.check_strategy_position_size(&call_name, &args);
        self.check_strategy_price_bounds(&call_name, &args);
        self.check_strategy_limit_stop_relation(&call_name, &args);
        self.check_strategy_exit_profit_loss(&call_name, &args);
        self.check_strategy_trail(&call_name, &args);
        self.check_strategy_position_management(&call_name, &args);
        self.check_strategy_alert_message(&call_name, &args);
        self.check_closedtrades_index(&call_name, &args);
        self.check_timeframe_call(&call_name, &args);
        self.check_chart_point_call(&call_name, &args);
        self.check_session_parameter(&call_name, &args);
        self.check_math_domain(&call_name, &args);
        self.check_na_usage(&call_name, &args);
        self.check_string_na_usage(&call_name, &args);
        self.check_ta_param_relationships(&call_name, &args);
        self.record_ta_length(&call_name, &args);
        self.check_plot_title(&call_name, &args);
        self.check_plot_style_consistency(&call_name, &args);
        self.check_color_values(&call_name, &args);
        self.check_deleted_draw_object_usage(&call_name, &args);
        self.check_map_operations(&call_name, &args);
        self.maybe_warn_repainting(&call_name, &args);

        if matches!(
            call_name.as_str(),
            "strategy.entry" | "strategy.order" | "strategy.exit" | "strategy.close"
        ) && let Some(context) = self.condition_stack.last_mut()
        {
            if call_name == "strategy.entry" || call_name == "strategy.order" {
                context.strategy_entry = true;
            }
            if call_name == "strategy.exit" || call_name == "strategy.close" {
                context.strategy_exit = true;
            }
        }

        match call_name.as_str() {
            "array.get" | "array.set" => self.check_array_bounds(&args, 1),
            "array.pop" | "array.shift" => self.check_array_empty_ops(&args),
            "array.remove" => {
                self.check_array_bounds(&args, 1);
                self.check_array_empty_ops(&args);
            }
            "matrix.get" | "matrix.set" => self.check_matrix_bounds(&args, 1, 2),
            "matrix.row" => self.check_matrix_index_bounds(&args, 1, true),
            "matrix.col" => self.check_matrix_index_bounds(&args, 1, false),
            "matrix.add_row" => self.check_matrix_index_bounds(&args, 1, true),
            "matrix.add_col" => self.check_matrix_index_bounds(&args, 1, false),
            "matrix.remove_row" => self.check_matrix_index_bounds(&args, 1, true),
            "matrix.remove_col" => self.check_matrix_index_bounds(&args, 1, false),
            "matrix.swap_rows" => self.check_matrix_swap_bounds(&args, 1, 2, true),
            "matrix.swap_columns" => self.check_matrix_swap_bounds(&args, 1, 2, false),
            "matrix.sum" | "matrix.diff" | "matrix.mult" | "matrix.concat" => {
                self.check_matrix_size_compat(&call_name, &args, range)
            }
            _ => {}
        }

        if matches!(
            call_name.as_str(),
            "array.push" | "array.set" | "array.insert" | "array.unshift" | "array.from"
        ) {
            self.check_array_type_consistency(&call_name, &args);
        }

        if matches!(call_name.as_str(), "matrix.set" | "matrix.fill") {
            self.check_matrix_type_consistency(&call_name, &args);
        }

        self.check_table_bounds(&call_name, &args);

        if let Some(current) = self.function_stack.last().cloned()
            && let Some(def_index) = self.resolve_symbol(&call_name)
        {
            let def = &self.definitions[def_index];
            if def.kind == SymbolKind::Function && !def.is_builtin {
                self.function_calls
                    .entry(current)
                    .or_default()
                    .insert(def.name.clone());
            }
        }

        let arg_nodes: Vec<Node> = args.iter().map(|(_, arg)| *arg).collect();
        let arg_types: Vec<Type> = arg_nodes
            .iter()
            .map(|arg| self.infer_expr_type(*arg))
            .collect();
        self.check_call(&call_name, &args, &arg_types);

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() != "identifier" {
                self.walk(child);
            }
        }
    }

    pub(super) fn handle_if_statement(&mut self, node: Node) {
        let condition = node.child_by_field_name("condition");
        if let Some(condition) = condition {
            self.walk(condition);
            let condition_range = self.range_for_node(condition);
            let condition_has_close_zero = self.expr_has_close_zero(condition);
            let condition_has_barstate = self.expr_has_barstate_flag(condition);
            self.check_constant_condition(condition);
            if condition_has_barstate {
                self.push_diagnostic(
                    condition_range,
                    DiagnosticSeverity::HINT,
                    "Verify barstate.isconfirmed/islast usage; behavior differs on historical bars"
                        .to_string(),
                    "logic".to_string(),
                );
            }
            if let Some(consequence) = node.child_by_field_name("consequence") {
                self.condition_stack.push(ConditionContext {
                    range: condition_range,
                    strategy_entry: false,
                    strategy_exit: false,
                    condition_has_close_zero,
                });
                self.walk(consequence);
                if let Some(context) = self.condition_stack.pop() {
                    self.finalize_condition_context(context);
                }
            }
        }

        for i in 0..node.child_count() {
            if let Some(child) = node.child(i as u32)
                && node.field_name_for_child(i as u32) == Some("alternative")
            {
                match child.kind() {
                    "else_if_clause" => self.handle_else_if_clause(child),
                    "else_clause" => self.handle_else_clause(child),
                    _ => self.walk(child),
                }
            }
        }
    }

    pub(super) fn handle_else_if_clause(&mut self, node: Node) {
        let condition = node.child_by_field_name("condition");
        if let Some(condition) = condition {
            self.walk(condition);
            let condition_range = self.range_for_node(condition);
            let condition_has_close_zero = self.expr_has_close_zero(condition);
            let condition_has_barstate = self.expr_has_barstate_flag(condition);
            self.check_constant_condition(condition);
            if condition_has_barstate {
                self.push_diagnostic(
                    condition_range,
                    DiagnosticSeverity::HINT,
                    "Verify barstate.isconfirmed/islast usage; behavior differs on historical bars"
                        .to_string(),
                    "logic".to_string(),
                );
            }
            if let Some(consequence) = node.child_by_field_name("consequence") {
                self.condition_stack.push(ConditionContext {
                    range: condition_range,
                    strategy_entry: false,
                    strategy_exit: false,
                    condition_has_close_zero,
                });
                self.walk(consequence);
                if let Some(context) = self.condition_stack.pop() {
                    self.finalize_condition_context(context);
                }
            }
        }
    }

    pub(super) fn handle_else_clause(&mut self, node: Node) {
        if let Some(consequence) = node.child_by_field_name("consequence") {
            let range = self.range_for_node(node);
            self.condition_stack.push(ConditionContext {
                range,
                strategy_entry: false,
                strategy_exit: false,
                condition_has_close_zero: false,
            });
            self.walk(consequence);
            if let Some(context) = self.condition_stack.pop() {
                self.finalize_condition_context(context);
            }
        }
    }

    pub(super) fn handle_while_statement(&mut self, node: Node) {
        self.check_while_statement(node);
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            self.walk(child);
        }
    }

    pub(super) fn collect_call_arguments(node: Node) -> Vec<(Option<Node>, Node)> {
        let mut args = Vec::new();
        if let Some(arg_list) = node.child_by_field_name("arguments") {
            for i in 0..arg_list.child_count() {
                if let Some(child) = arg_list.child(i as u32) {
                    if child.kind() == "keyword_argument" {
                        if let Some(value) = child.child_by_field_name("value") {
                            let name_node = child.child_by_field_name("key");
                            args.push((name_node, value));
                        }
                    } else if child.is_named() {
                        args.push((None, child));
                    }
                }
            }
        }
        args
    }
}
