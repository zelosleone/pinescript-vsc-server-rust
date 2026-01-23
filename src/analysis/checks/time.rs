use tower_lsp::lsp_types::DiagnosticSeverity;
use tree_sitter::Node;

use crate::analysis::Analyzer;

impl<'a> Analyzer<'a> {
    pub(in crate::analysis) fn check_timeframe_call(
        &mut self,
        name: &str,
        args: &[(Option<String>, Node)],
    ) {
        if name != "timeframe.change" && name != "timeframe.in_seconds" {
            return;
        }
        if !self.settings.enable_timeframe_format_validation {
            return;
        }
        let Some((tf_node, _idx)) = Self::find_call_arg(args, "timeframe", 0) else {
            return;
        };
        let Some(value) = self.eval_string_constant(tf_node) else {
            self.push_diagnostic(
                self.range_for_node(tf_node),
                DiagnosticSeverity::HINT,
                "timeframe parameter should be a constant string".to_string(),
                "logic".to_string(),
            );
            return;
        };
        if value.trim().is_empty() {
            self.push_diagnostic(
                self.range_for_node(tf_node),
                DiagnosticSeverity::HINT,
                "timeframe parameter is empty".to_string(),
                "logic".to_string(),
            );
            return;
        }
        if !self.is_valid_timeframe_string(&value) {
            self.push_diagnostic(
                self.range_for_node(tf_node),
                DiagnosticSeverity::HINT,
                format!("Invalid timeframe format `{}`", value),
                "logic".to_string(),
            );
        }
    }

    pub(in crate::analysis) fn check_chart_point_call(
        &mut self,
        name: &str,
        args: &[(Option<String>, Node)],
    ) {
        if !self.settings.enable_chart_point_validation {
            return;
        }
        match name {
            "chart.point.from_index" => {
                let Some((idx_node, _idx)) = Self::find_call_arg(args, "index", 0) else {
                    return;
                };
                let Some(value) = self.eval_numeric_constant(idx_node) else {
                    return;
                };
                if value < 0.0 {
                    self.push_diagnostic(
                        self.range_for_node(idx_node),
                        DiagnosticSeverity::HINT,
                        "chart.point.from_index index should be >= 0".to_string(),
                        "logic".to_string(),
                    );
                    return;
                }
                let threshold = self.settings.chart_point_index_warn_threshold;
                if threshold > 0 && value.round() as i64 >= threshold {
                    self.push_diagnostic(
                        self.range_for_node(idx_node),
                        DiagnosticSeverity::HINT,
                        "chart.point.from_index index is unusually large".to_string(),
                        "logic".to_string(),
                    );
                }
            }
            "chart.point.from_time" => {
                let Some((time_node, _idx)) = Self::find_call_arg(args, "time", 0) else {
                    return;
                };
                let Some(value) = self.eval_numeric_constant(time_node) else {
                    return;
                };
                if value <= 0.0 {
                    self.push_diagnostic(
                        self.range_for_node(time_node),
                        DiagnosticSeverity::HINT,
                        "chart.point.from_time expects a positive timestamp".to_string(),
                        "logic".to_string(),
                    );
                }
            }
            _ => {}
        }
    }

    pub(in crate::analysis) fn check_session_parameter(
        &mut self,
        name: &str,
        args: &[(Option<String>, Node)],
    ) {
        if !self.settings.enable_session_validation {
            return;
        }
        let session_node = match name {
            "input.session" => Self::find_call_arg(args, "defval", 0).map(|(node, _)| node),
            "time" | "time_close" => Self::find_call_arg(args, "session", 1).map(|(node, _)| node),
            _ => None,
        };
        let Some(session_node) = session_node else {
            return;
        };
        let Some(value) = self.eval_string_constant(session_node) else {
            return;
        };
        if value.trim().is_empty() {
            self.push_diagnostic(
                self.range_for_node(session_node),
                DiagnosticSeverity::HINT,
                "Session string is empty".to_string(),
                "logic".to_string(),
            );
            return;
        }
        if !self.is_valid_session_string(&value) {
            self.push_diagnostic(
                self.range_for_node(session_node),
                DiagnosticSeverity::HINT,
                "Session string format is invalid".to_string(),
                "logic".to_string(),
            );
        }
    }

    pub(in crate::analysis) fn is_valid_timeframe_string(&self, value: &str) -> bool {
        let value = value.trim();
        if value.is_empty() {
            return false;
        }
        if matches!(value, "D" | "W" | "M") {
            return true;
        }
        let mut chars = value.chars();
        let mut digits = String::new();
        let mut suffix = None;
        while let Some(ch) = chars.next() {
            if ch.is_ascii_digit() {
                digits.push(ch);
            } else {
                suffix = Some(ch);
                if chars.next().is_some() {
                    return false;
                }
                break;
            }
        }
        if digits.is_empty() {
            return false;
        }
        if let Some(suffix) = suffix {
            matches!(suffix, 'S' | 'H' | 'D' | 'W' | 'M')
        } else {
            true
        }
    }

    pub(in crate::analysis) fn is_valid_session_string(&self, value: &str) -> bool {
        let value = value.trim();
        if value.is_empty() {
            return false;
        }
        if value == "24x7" {
            return true;
        }
        let mut parts = value.split(':');
        let session_part = parts.next().unwrap_or("");
        let days_part = parts.next();
        if parts.next().is_some() {
            return false;
        }

        let mut range_iter = session_part.split('-');
        let start = range_iter.next().unwrap_or("");
        let end = range_iter.next().unwrap_or("");
        if range_iter.next().is_some() {
            return false;
        }
        if start.len() != 4 || end.len() != 4 {
            return false;
        }
        let parse_hhmm = |s: &str| -> Option<(u32, u32)> {
            let hh = s.get(0..2)?.parse::<u32>().ok()?;
            let mm = s.get(2..4)?.parse::<u32>().ok()?;
            if hh > 23 || mm > 59 {
                return None;
            }
            Some((hh, mm))
        };
        if parse_hhmm(start).is_none() || parse_hhmm(end).is_none() {
            return false;
        }
        if let Some(days) = days_part {
            if days.is_empty() || days.len() > 7 {
                return false;
            }
            if !days.chars().all(|ch| matches!(ch, '1'..='7')) {
                return false;
            }
        }
        true
    }
}
