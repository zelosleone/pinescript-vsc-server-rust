use tower_lsp::lsp_types::{DiagnosticSeverity, Range};
use tree_sitter::Node;

use crate::analysis::Analyzer;

impl<'a> Analyzer<'a> {
    pub(in crate::analysis) fn check_input_range(
        &mut self,
        name: &str,
        args: &[(Option<String>, Node)],
    ) {
        if !self.settings.enable_input_range_validation {
            return;
        }
        if name != "input.int" && name != "input.float" {
            return;
        }
        let def_node = Self::find_call_arg(args, "defval", 0).map(|(node, _)| node);
        let min_node = Self::find_call_arg(args, "minval", 2).map(|(node, _)| node);
        let max_node = Self::find_call_arg(args, "maxval", 3).map(|(node, _)| node);

        let min_val = min_node.and_then(|node| self.eval_numeric_constant(node));
        let max_val = max_node.and_then(|node| self.eval_numeric_constant(node));

        if let (Some(min_val), Some(max_val)) = (min_val, max_val)
            && min_val >= max_val
        {
            let range = min_node
                .map(|node| self.range_for_node(node))
                .unwrap_or_else(Range::default);
            self.push_diagnostic(
                range,
                DiagnosticSeverity::HINT,
                "input range is invalid; minval should be < maxval".to_string(),
                "logic".to_string(),
            );
        }

        if let Some(def_node) = def_node
            && let Some(def_val) = self.eval_numeric_constant(def_node)
        {
            if let Some(min_val) = min_val
                && def_val < min_val
            {
                self.push_diagnostic(
                    self.range_for_node(def_node),
                    DiagnosticSeverity::HINT,
                    "input defval is below minval".to_string(),
                    "logic".to_string(),
                );
            }
            if let Some(max_val) = max_val
                && def_val > max_val
            {
                self.push_diagnostic(
                    self.range_for_node(def_node),
                    DiagnosticSeverity::HINT,
                    "input defval exceeds maxval".to_string(),
                    "logic".to_string(),
                );
            }
        }
    }
}
