use std::collections::HashSet;

use tower_lsp::lsp_types::{DiagnosticSeverity, Range};
use tree_sitter::Node;

use crate::analysis::{Analyzer, SymbolKind};

impl<'a> Analyzer<'a> {
    pub(in crate::analysis) fn check_history_reference(&mut self, node: Node) {
        let Some(offset) = node.child_by_field_name("offset") else {
            return;
        };
        let Some(value) = self.eval_numeric_constant(offset) else {
            return;
        };
        if value < 0.0 {
            self.push_diagnostic(
                self.range_for_node(offset),
                DiagnosticSeverity::HINT,
                "Possible lookahead bias: negative history reference uses future bars".to_string(),
                "logic".to_string(),
            );
            return;
        }

        let offset_value = value.round() as i64;
        if let Some(max_bars_back) = self.max_bars_back {
            if offset_value > max_bars_back {
                self.push_diagnostic(
                    self.range_for_node(offset),
                    DiagnosticSeverity::HINT,
                    format!(
                        "History reference {} exceeds max_bars_back {}",
                        offset_value, max_bars_back
                    ),
                    "logic".to_string(),
                );
            }
            return;
        }

        let threshold = self.settings.history_reference_warn_threshold;
        if threshold > 0 && offset_value >= threshold {
            self.push_diagnostic(
                self.range_for_node(offset),
                DiagnosticSeverity::HINT,
                format!(
                    "History reference {} may exceed available bars; consider max_bars_back",
                    offset_value
                ),
                "logic".to_string(),
            );
        }
    }

    pub(in crate::analysis) fn update_max_bars_back(
        &mut self,
        name: &str,
        args: &[(Option<String>, Node)],
    ) {
        let max_node = if name == "max_bars_back" {
            Self::find_call_arg(args, "length", 1).map(|(node, _idx)| node)
        } else if name == "indicator" || name == "strategy" {
            Self::find_call_arg_by_name(args, "max_bars_back")
        } else {
            None
        };

        let Some(max_node) = max_node else {
            return;
        };
        let Some(value) = self.eval_numeric_constant(max_node) else {
            return;
        };
        if value <= 0.0 {
            return;
        }
        let value = value.round() as i64;
        self.max_bars_back = Some(match self.max_bars_back {
            Some(existing) => existing.max(value),
            None => value,
        });
    }

    pub(in crate::analysis) fn check_for_statement_bounds(&mut self, node: Node) {
        let Some(from_node) = node.child_by_field_name("from_num") else {
            return;
        };
        let Some(to_node) = node.child_by_field_name("to_num") else {
            return;
        };
        let from_val = self.eval_numeric_constant(from_node);
        let to_val = self.eval_numeric_constant(to_node);
        let step_node = node.child_by_field_name("step_num");
        let step_val = step_node.and_then(|node| self.eval_numeric_constant(node));

        if let Some(step_val) = step_val
            && step_val == 0.0
        {
            self.push_diagnostic(
                self.range_for_node(step_node.unwrap()),
                DiagnosticSeverity::HINT,
                "Loop step is zero; loop will not terminate".to_string(),
                "logic".to_string(),
            );
            return;
        }

        let (Some(from_val), Some(to_val)) = (from_val, to_val) else {
            return;
        };
        let steps = if let Some(step_val) = step_val {
            if step_val == 0.0 {
                return;
            }
            ((to_val - from_val).abs() / step_val.abs()).round()
        } else {
            (to_val - from_val).abs().round()
        };
        let threshold = self.settings.loop_large_threshold;
        if threshold > 0 && steps >= threshold as f64 {
            self.push_diagnostic(
                self.range_for_node(node),
                DiagnosticSeverity::HINT,
                "Loop range is very large; this may be slow".to_string(),
                "logic".to_string(),
            );
        }
    }

    pub(in crate::analysis) fn check_while_statement(&mut self, node: Node) {
        let Some(condition) = node.child_by_field_name("condition") else {
            return;
        };
        if !self.settings.enable_constant_condition_warnings {
            return;
        }
        if let Some(value) = self.eval_bool_constant(condition) {
            let message = if value {
                "While loop condition is always true; ensure it terminates"
            } else {
                "While loop condition is always false; loop will never execute"
            };
            self.push_diagnostic(
                self.range_for_node(condition),
                DiagnosticSeverity::HINT,
                message.to_string(),
                "logic".to_string(),
            );
        }
    }

    pub(in crate::analysis) fn check_constant_condition(&mut self, condition: Node) {
        if !self.settings.enable_constant_condition_warnings {
            return;
        }
        let Some(value) = self.eval_bool_constant(condition) else {
            return;
        };
        let message = if value {
            "Condition is always true"
        } else {
            "Condition is always false"
        };
        self.push_diagnostic(
            self.range_for_node(condition),
            DiagnosticSeverity::HINT,
            message.to_string(),
            "logic".to_string(),
        );
    }

    pub(in crate::analysis) fn check_duplicate_expression(
        &mut self,
        expr: Node,
        selection_range: Range,
    ) {
        let expr_text = self
            .text
            .get(expr.byte_range())
            .unwrap_or("")
            .split_whitespace()
            .collect::<String>();
        if expr_text.len() < 8 {
            return;
        }
        let existing = self.duplicate_exprs.get(&expr_text).cloned();
        if let Some(existing_range) = existing {
            self.push_diagnostic(
                selection_range,
                DiagnosticSeverity::WARNING,
                "Duplicate expression; consider extracting an abstraction".to_string(),
                "style".to_string(),
            );
            self.push_diagnostic(
                existing_range,
                DiagnosticSeverity::INFORMATION,
                "Previous duplicate expression".to_string(),
                "style".to_string(),
            );
        } else {
            self.duplicate_exprs.insert(expr_text, selection_range);
        }
    }

    pub(in crate::analysis) fn check_version_directive(&mut self) {
        let has_version = self
            .text
            .lines()
            .any(|line| line.trim_start().starts_with("//@version=6"));
        if !has_version {
            self.push_diagnostic(
                Range::default(),
                DiagnosticSeverity::WARNING,
                "Missing //@version=6 directive".to_string(),
                "validation".to_string(),
            );
        }
    }

    pub(in crate::analysis) fn check_entrypoint(&mut self) {
        if !self.has_indicator_or_strategy {
            self.push_diagnostic(
                Range::default(),
                DiagnosticSeverity::WARNING,
                "Missing indicator() or strategy() declaration".to_string(),
                "validation".to_string(),
            );
        }
    }

    pub(in crate::analysis) fn report_recursive_functions(&mut self) {
        if !self.settings.enable_function_recursion_detection {
            return;
        }
        let functions: Vec<String> = self.function_calls.keys().cloned().collect();
        let mut reported = HashSet::new();
        for func in functions {
            let mut visiting = HashSet::new();
            if self.has_recursive_path(&func, &func, &mut visiting) {
                if !reported.insert(func.clone()) {
                    continue;
                }
                if let Some(range) = self.function_definition_range(&func) {
                    self.push_diagnostic(
                        range,
                        DiagnosticSeverity::HINT,
                        format!(
                            "Function `{}` is recursive; ensure a base case to avoid infinite recursion",
                            func
                        ),
                        "logic".to_string(),
                    );
                }
            }
        }
    }

    fn has_recursive_path(
        &self,
        start: &str,
        current: &str,
        visiting: &mut HashSet<String>,
    ) -> bool {
        if !visiting.insert(current.to_string()) {
            return false;
        }
        let Some(callees) = self.function_calls.get(current) else {
            visiting.remove(current);
            return false;
        };
        for callee in callees {
            if callee == start {
                return true;
            }
            if self.has_recursive_path(start, callee, visiting) {
                return true;
            }
        }
        visiting.remove(current);
        false
    }

    fn function_definition_range(&self, name: &str) -> Option<Range> {
        self.definitions
            .iter()
            .find(|def| def.kind == SymbolKind::Function && def.name == name)
            .map(|def| def.selection_range)
    }
}
