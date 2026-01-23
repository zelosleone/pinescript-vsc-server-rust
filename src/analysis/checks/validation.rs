use tower_lsp::lsp_types::DiagnosticSeverity;
use tree_sitter::Node;

use crate::analysis::Analyzer;

impl<'a> Analyzer<'a> {
    pub(in crate::analysis) fn check_na_usage(
        &mut self,
        name: &str,
        args: &[(Option<String>, Node)],
    ) {
        if !name.starts_with("ta.") && !name.starts_with("math.") {
            return;
        }
        for (_opt_name, node) in args {
            if self.expr_contains_na(*node) {
                self.push_diagnostic(
                    self.range_for_node(*node),
                    DiagnosticSeverity::HINT,
                    "Argument may be na; guard with na() or nz()".to_string(),
                    "logic".to_string(),
                );
            }
        }
    }

    pub(in crate::analysis) fn check_color_values(
        &mut self,
        name: &str,
        args: &[(Option<String>, Node)],
    ) {
        if !self.settings.enable_color_value_validation {
            return;
        }
        let mut check_range = |node: Node, label: &str, min: f64, max: f64| {
            if let Some(value) = self.eval_numeric_constant(node)
                && (value < min || value > max)
            {
                self.push_diagnostic(
                    self.range_for_node(node),
                    DiagnosticSeverity::HINT,
                    format!(
                        "{} value {} is out of range [{}, {}]",
                        label, value, min, max
                    ),
                    "logic".to_string(),
                );
            }
        };

        match name {
            "color.rgb" => {
                if let Some((node, _idx)) = Self::find_call_arg(args, "red", 0) {
                    check_range(node, "red", 0.0, 255.0);
                }
                if let Some((node, _idx)) = Self::find_call_arg(args, "green", 1) {
                    check_range(node, "green", 0.0, 255.0);
                }
                if let Some((node, _idx)) = Self::find_call_arg(args, "blue", 2) {
                    check_range(node, "blue", 0.0, 255.0);
                }
                if let Some((node, _idx)) = Self::find_call_arg(args, "transp", 3) {
                    check_range(node, "transp", 0.0, 100.0);
                }
            }
            "color.new" => {
                if let Some((node, _idx)) = Self::find_call_arg(args, "transp", 1) {
                    check_range(node, "transp", 0.0, 100.0);
                }
            }
            _ => {}
        }
    }

    pub(in crate::analysis) fn check_string_na_usage(
        &mut self,
        name: &str,
        args: &[(Option<String>, Node)],
    ) {
        if !name.starts_with("str.") {
            return;
        }
        if !self.settings.enable_string_na_checks {
            return;
        }
        for (_opt_name, node) in args {
            if self.expr_contains_na(*node) {
                self.push_diagnostic(
                    self.range_for_node(*node),
                    DiagnosticSeverity::HINT,
                    "String function argument may be na".to_string(),
                    "logic".to_string(),
                );
            }
        }
    }
}
