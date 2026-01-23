use tower_lsp::lsp_types::{DiagnosticSeverity, Range};
use tree_sitter::Node;

use crate::analysis::Analyzer;

impl<'a> Analyzer<'a> {
    pub(in crate::analysis) fn check_request_security_lookahead(
        &mut self,
        name: &str,
        args: &[(Option<String>, Node)],
        call_range: Range,
    ) {
        if name != "request.security" {
            return;
        }
        let lookahead_arg = Self::find_call_arg(args, "lookahead", 4);
        let Some((lookahead_node, _idx)) = lookahead_arg else {
            self.push_diagnostic(
                call_range,
                DiagnosticSeverity::HINT,
                "request.security missing `lookahead`; default may allow lookahead bias"
                    .to_string(),
                "logic".to_string(),
            );
            return;
        };

        let value_name = if lookahead_node.kind() == "attribute" {
            self.attribute_chain_name(lookahead_node)
        } else if lookahead_node.kind() == "identifier" {
            Some(self.node_text(lookahead_node))
        } else {
            None
        };
        if let Some(value_name) = value_name
            && value_name == "barmerge.lookahead_on"
        {
            self.push_diagnostic(
                self.range_for_node(lookahead_node),
                DiagnosticSeverity::HINT,
                "request.security lookahead set to barmerge.lookahead_on; this can introduce lookahead bias".to_string(),
                "logic".to_string(),
            );
        }
    }

    pub(in crate::analysis) fn check_request_security_gaps(
        &mut self,
        name: &str,
        args: &[(Option<String>, Node)],
        call_range: Range,
    ) {
        if name != "request.security" {
            return;
        }
        if Self::find_call_arg(args, "gaps", 3).is_some() {
            return;
        }
        self.push_diagnostic(
            call_range,
            DiagnosticSeverity::HINT,
            "request.security missing `gaps` parameter".to_string(),
            "logic".to_string(),
        );
    }

    pub(in crate::analysis) fn check_request_security_expression(
        &mut self,
        name: &str,
        args: &[(Option<String>, Node)],
        call_range: Range,
    ) {
        if name != "request.security" {
            return;
        }
        if !self.settings.enable_request_security_expression_validation {
            return;
        }
        let expr_node = Self::find_call_arg(args, "expression", 2).map(|(node, _)| node);
        let Some(expr_node) = expr_node else {
            self.push_diagnostic(
                call_range,
                DiagnosticSeverity::HINT,
                "request.security missing expression parameter".to_string(),
                "logic".to_string(),
            );
            return;
        };
        if expr_node.is_missing() || expr_node.is_error() {
            self.push_diagnostic(
                self.range_for_node(expr_node),
                DiagnosticSeverity::HINT,
                "request.security expression is invalid".to_string(),
                "logic".to_string(),
            );
        }
    }

    pub(in crate::analysis) fn check_request_security_format(
        &mut self,
        name: &str,
        args: &[(Option<String>, Node)],
        call_range: Range,
    ) {
        if name != "request.security" {
            return;
        }
        if !self.settings.enable_request_security_format_validation {
            return;
        }
        let symbol_node = Self::find_call_arg(args, "symbol", 0).map(|(node, _)| node);
        if let Some(symbol_node) = symbol_node
            && let Some(symbol) = self.eval_string_constant(symbol_node)
            && symbol.trim().is_empty()
        {
            self.push_diagnostic(
                self.range_for_node(symbol_node),
                DiagnosticSeverity::HINT,
                "request.security symbol is empty".to_string(),
                "logic".to_string(),
            );
        }

        let timeframe_node = Self::find_call_arg(args, "timeframe", 1).map(|(node, _)| node);
        if let Some(timeframe_node) = timeframe_node {
            let Some(tf) = self.eval_string_constant(timeframe_node) else {
                self.push_diagnostic(
                    self.range_for_node(timeframe_node),
                    DiagnosticSeverity::HINT,
                    "request.security timeframe should be a constant string".to_string(),
                    "logic".to_string(),
                );
                return;
            };
            if !self.is_valid_timeframe_string(&tf) {
                self.push_diagnostic(
                    self.range_for_node(timeframe_node),
                    DiagnosticSeverity::HINT,
                    format!("Invalid timeframe format `{}`", tf),
                    "logic".to_string(),
                );
            }
        } else {
            self.push_diagnostic(
                call_range,
                DiagnosticSeverity::HINT,
                "request.security missing timeframe parameter".to_string(),
                "logic".to_string(),
            );
        }
    }
}
