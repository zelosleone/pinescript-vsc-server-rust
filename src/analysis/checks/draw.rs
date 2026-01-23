use tower_lsp::lsp_types::{DiagnosticSeverity, Range};
use tree_sitter::Node;

use crate::analysis::Analyzer;

impl<'a> Analyzer<'a> {
    pub(in crate::analysis) fn record_draw_object_assignment(
        &mut self,
        var_name: &str,
        expr: Node,
        range: Range,
    ) {
        if !self.settings.enable_draw_object_id_reuse {
            return;
        }
        if expr.kind() != "call" {
            return;
        }
        let Some(function_node) = expr.child_by_field_name("function") else {
            return;
        };
        let call_name = self.call_name_from_node(function_node);
        let kind = match call_name.as_str() {
            "label.new" => "label",
            "line.new" => "line",
            "box.new" => "box",
            "table.new" => "table",
            _ => return,
        };

        self.deleted_draw_objects.remove(var_name);

        if let Some((prev_kind, prev_range)) = self.draw_object_vars.get(var_name).cloned() {
            self.push_diagnostic(
                range,
                DiagnosticSeverity::HINT,
                format!(
                    "Drawing object id `{}` reused for multiple {} objects",
                    var_name, kind
                ),
                "logic".to_string(),
            );
            self.push_diagnostic(
                prev_range,
                DiagnosticSeverity::INFORMATION,
                format!("Previous `{}` assignment here", prev_kind),
                "logic".to_string(),
            );
        } else {
            self.draw_object_vars
                .insert(var_name.to_string(), (kind.to_string(), range));
        }
    }

    pub(in crate::analysis) fn check_deleted_draw_object_usage(
        &mut self,
        name: &str,
        args: &[(Option<String>, Node)],
    ) {
        if !self.settings.enable_draw_object_lifecycle_checks {
            return;
        }
        let kind = if name.starts_with("label.") {
            "label"
        } else if name.starts_with("line.") {
            "line"
        } else if name.starts_with("box.") {
            "box"
        } else if name.starts_with("table.") {
            "table"
        } else {
            return;
        };
        if name.ends_with(".new") {
            return;
        }
        let Some((id_node, _idx)) = Self::find_call_arg(args, "id", 0) else {
            return;
        };
        let Some(id_name) = self.extract_identifier_name(id_node) else {
            return;
        };

        if name.ends_with(".delete") {
            self.deleted_draw_objects
                .insert(id_name, (kind.to_string(), self.range_for_node(id_node)));
            return;
        }

        if let Some((deleted_kind, deleted_range)) =
            self.deleted_draw_objects.get(&id_name).cloned()
        {
            self.push_diagnostic(
                self.range_for_node(id_node),
                DiagnosticSeverity::HINT,
                format!("Use of deleted {} id `{}`", deleted_kind, id_name),
                "logic".to_string(),
            );
            self.push_diagnostic(
                deleted_range,
                DiagnosticSeverity::INFORMATION,
                "Deleted here".to_string(),
                "logic".to_string(),
            );
        }
    }
}
