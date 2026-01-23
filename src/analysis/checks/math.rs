use tower_lsp::lsp_types::DiagnosticSeverity;
use tree_sitter::Node;

use crate::analysis::Analyzer;

impl<'a> Analyzer<'a> {
    pub(in crate::analysis) fn check_math_operation(&mut self, node: Node) {
        let Some(operator) = node.child_by_field_name("operator") else {
            return;
        };
        let op = self.node_text(operator);
        if op != "/" && op != "%" {
            return;
        }
        let Some(right) = node.child_by_field_name("right") else {
            return;
        };
        if self.expr_is_zero(right) {
            self.push_diagnostic(
                self.range_for_node(right),
                DiagnosticSeverity::HINT,
                "Division by zero; divisor evaluates to 0".to_string(),
                "logic".to_string(),
            );
            return;
        }
        if self.expr_contains_na(right) {
            self.push_diagnostic(
                self.range_for_node(right),
                DiagnosticSeverity::HINT,
                "Division by na may propagate na values".to_string(),
                "logic".to_string(),
            );
        }
    }

    pub(in crate::analysis) fn check_math_domain(
        &mut self,
        name: &str,
        args: &[(Option<String>, Node)],
    ) {
        let Some((arg_node, _idx)) = Self::find_call_arg(args, "x", 0) else {
            return;
        };
        let arg_val = self.eval_numeric_constant(arg_node);

        match name {
            "math.log" | "math.log10" => {
                if self.expr_is_zero(arg_node) {
                    self.push_diagnostic(
                        self.range_for_node(arg_node),
                        DiagnosticSeverity::HINT,
                        "math.log input is 0; result is undefined".to_string(),
                        "logic".to_string(),
                    );
                    return;
                }
                if let Some(val) = arg_val
                    && val < 0.0
                {
                    self.push_diagnostic(
                        self.range_for_node(arg_node),
                        DiagnosticSeverity::HINT,
                        "math.log input is negative; result is undefined".to_string(),
                        "logic".to_string(),
                    );
                }
            }
            "math.sqrt" => {
                if let Some(val) = arg_val
                    && val < 0.0
                {
                    self.push_diagnostic(
                        self.range_for_node(arg_node),
                        DiagnosticSeverity::HINT,
                        "math.sqrt input is negative; result is NaN".to_string(),
                        "logic".to_string(),
                    );
                }
            }
            "math.asin" | "math.acos" => {
                if let Some(val) = arg_val
                    && !(-1.0..=1.0).contains(&val)
                {
                    self.push_diagnostic(
                        self.range_for_node(arg_node),
                        DiagnosticSeverity::HINT,
                        format!("{} input out of [-1, 1] range", name),
                        "logic".to_string(),
                    );
                }
            }
            _ => {}
        }
    }
}
