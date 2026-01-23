use tower_lsp::lsp_types::DiagnosticSeverity;
use tree_sitter::Node;

use crate::analysis::Analyzer;

impl<'a> Analyzer<'a> {
    pub(in crate::analysis) fn check_ta_param_relationships(
        &mut self,
        name: &str,
        args: &[(Option<String>, Node)],
    ) {
        if name == "ta.pivothigh" || name == "ta.pivotlow" {
            if !self.settings.enable_pivot_validation {
                return;
            }
        } else if !self.settings.enable_ta_param_relationships {
            return;
        }

        match name {
            "ta.stoch" => {
                let k = Self::find_call_arg(args, "k", 0)
                    .and_then(|(node, _)| self.eval_numeric_constant(node));
                let d = Self::find_call_arg(args, "d", 1)
                    .and_then(|(node, _)| self.eval_numeric_constant(node));
                if let (Some(k), Some(d)) = (k, d)
                    && k >= d
                {
                    self.push_diagnostic(
                        self.range_for_node(Self::find_call_arg(args, "k", 0).unwrap().0),
                        DiagnosticSeverity::HINT,
                        "ta.stoch expects k < d".to_string(),
                        "logic".to_string(),
                    );
                }
            }
            "ta.macd" => {
                let fast = Self::find_call_arg(args, "fastlen", 1)
                    .and_then(|(node, _)| self.eval_numeric_constant(node));
                let slow = Self::find_call_arg(args, "slowlen", 2)
                    .and_then(|(node, _)| self.eval_numeric_constant(node));
                if let (Some(fast), Some(slow)) = (fast, slow)
                    && fast >= slow
                {
                    self.push_diagnostic(
                        self.range_for_node(Self::find_call_arg(args, "fastlen", 1).unwrap().0),
                        DiagnosticSeverity::HINT,
                        "ta.macd expects fastlen < slowlen".to_string(),
                        "logic".to_string(),
                    );
                }
            }
            "ta.bb" | "ta.kc" | "ta.bbw" | "ta.kcw" => {
                let mult = Self::find_call_arg(args, "mult", 2)
                    .and_then(|(node, _)| self.eval_numeric_constant(node));
                if let Some(mult) = mult
                    && mult <= 0.0
                {
                    self.push_diagnostic(
                        self.range_for_node(Self::find_call_arg(args, "mult", 2).unwrap().0),
                        DiagnosticSeverity::HINT,
                        "Multiplier should be > 0".to_string(),
                        "logic".to_string(),
                    );
                }
            }
            "ta.sar" => {
                let start = Self::find_call_arg(args, "start", 0)
                    .and_then(|(node, _)| self.eval_numeric_constant(node));
                let inc = Self::find_call_arg(args, "inc", 1)
                    .and_then(|(node, _)| self.eval_numeric_constant(node));
                let max = Self::find_call_arg(args, "max", 2)
                    .and_then(|(node, _)| self.eval_numeric_constant(node));
                if let (Some(start), Some(inc)) = (start, inc)
                    && start >= inc
                {
                    self.push_diagnostic(
                        self.range_for_node(Self::find_call_arg(args, "start", 0).unwrap().0),
                        DiagnosticSeverity::HINT,
                        "ta.sar expects start < inc".to_string(),
                        "logic".to_string(),
                    );
                }
                if let (Some(inc), Some(max)) = (inc, max)
                    && inc >= max
                {
                    self.push_diagnostic(
                        self.range_for_node(Self::find_call_arg(args, "inc", 1).unwrap().0),
                        DiagnosticSeverity::HINT,
                        "ta.sar expects inc < max".to_string(),
                        "logic".to_string(),
                    );
                }
            }
            "ta.percentile_linear_interpolation" | "ta.percentile_nearest_rank" => {
                let percentile = Self::find_call_arg(args, "percentile", 2)
                    .and_then(|(node, _)| self.eval_numeric_constant(node));
                if let Some(percentile) = percentile
                    && !(0.0..=100.0).contains(&percentile)
                {
                    self.push_diagnostic(
                        self.range_for_node(Self::find_call_arg(args, "percentile", 2).unwrap().0),
                        DiagnosticSeverity::HINT,
                        "Percentile should be in [0, 100]".to_string(),
                        "logic".to_string(),
                    );
                }
            }
            "ta.dmi" => {
                let length = Self::find_call_arg(args, "length", 0)
                    .and_then(|(node, _)| self.eval_numeric_constant(node));
                let lensig = Self::find_call_arg(args, "lensig", 1)
                    .and_then(|(node, _)| self.eval_numeric_constant(node));
                if let (Some(length), Some(lensig)) = (length, lensig)
                    && lensig >= length
                {
                    self.push_diagnostic(
                        self.range_for_node(Self::find_call_arg(args, "lensig", 1).unwrap().0),
                        DiagnosticSeverity::HINT,
                        "ta.dmi expects lensig < length".to_string(),
                        "logic".to_string(),
                    );
                }
            }
            "ta.pivothigh" | "ta.pivotlow" => {
                let mut left = Self::find_call_arg(args, "leftbars", 1)
                    .or_else(|| Self::find_call_arg(args, "leftbars", 0));
                let mut right = Self::find_call_arg(args, "rightbars", 2)
                    .or_else(|| Self::find_call_arg(args, "rightbars", 1));
                if left.is_none() || right.is_none() {
                    if args.len() >= 3 {
                        left = Some((args[1].1, 1));
                        right = Some((args[2].1, 2));
                    } else if args.len() >= 2 {
                        left = Some((args[0].1, 0));
                        right = Some((args[1].1, 1));
                    }
                }
                if let Some((left_node, _)) = left
                    && let Some(value) = self.eval_numeric_constant(left_node)
                    && value <= 0.0
                {
                    self.push_diagnostic(
                        self.range_for_node(left_node),
                        DiagnosticSeverity::HINT,
                        "leftbars should be > 0".to_string(),
                        "logic".to_string(),
                    );
                }
                if let Some((right_node, _)) = right
                    && let Some(value) = self.eval_numeric_constant(right_node)
                    && value <= 0.0
                {
                    self.push_diagnostic(
                        self.range_for_node(right_node),
                        DiagnosticSeverity::HINT,
                        "rightbars should be > 0".to_string(),
                        "logic".to_string(),
                    );
                }
            }
            _ => {}
        }
    }

    pub(in crate::analysis) fn record_ta_length(
        &mut self,
        name: &str,
        args: &[(Option<String>, Node)],
    ) {
        if !self.settings.enable_ta_length_consistency {
            return;
        }
        let length_node = match name {
            "ta.sma" | "ta.ema" | "ta.wma" | "ta.rma" | "ta.hma" | "ta.vwma" | "ta.rsi"
            | "ta.stdev" | "ta.highest" | "ta.lowest" | "ta.highestbars" | "ta.lowestbars"
            | "ta.cci" | "ta.roc" | "ta.mom" | "ta.dev" | "ta.median" | "ta.mode"
            | "ta.percentrank" | "ta.range" | "ta.rci" | "ta.variance" | "ta.wpr" | "ta.bbw"
            | "ta.kcw" | "ta.bb" | "ta.kc" => {
                Self::find_call_arg(args, "length", 1).map(|(node, _)| node)
            }
            "ta.atr" | "ta.adx" | "ta.mfi" | "ta.williamsr" => {
                Self::find_call_arg(args, "length", 0).map(|(node, _)| node)
            }
            _ => None,
        };
        let Some(length_node) = length_node else {
            return;
        };
        let Some(length) = self.eval_numeric_constant(length_node) else {
            return;
        };
        if length <= 0.0 {
            return;
        }
        let length = length.round() as i64;
        let range = self.range_for_node(length_node);
        let entry = self
            .ta_length_stats
            .entry(name.to_string())
            .or_insert((length, range, length, range));
        if length < entry.0 {
            entry.0 = length;
            entry.1 = range;
        }
        if length > entry.2 {
            entry.2 = length;
            entry.3 = range;
        }
    }

    pub(in crate::analysis) fn report_ta_length_inconsistency(&mut self) {
        if !self.settings.enable_ta_length_consistency {
            return;
        }
        let ratio = self.settings.ta_length_spread_ratio;
        let delta = self.settings.ta_length_spread_delta;
        if ratio <= 0 || delta <= 0 {
            return;
        }
        let mut hints = Vec::new();
        for (name, (min_len, min_range, max_len, max_range)) in &self.ta_length_stats {
            if *min_len <= 0 || *max_len <= 0 {
                continue;
            }
            if *max_len >= *min_len * ratio && (*max_len - *min_len) >= delta {
                hints.push((name.clone(), *min_len, *max_len, *min_range, *max_range));
            }
        }
        for (name, min_len, max_len, min_range, max_range) in hints {
            self.push_diagnostic(
                max_range,
                DiagnosticSeverity::HINT,
                format!("Large {} length spread ({} vs {})", name, min_len, max_len),
                "logic".to_string(),
            );
            self.push_diagnostic(
                min_range,
                DiagnosticSeverity::INFORMATION,
                format!("Smaller {} length used here", name),
                "logic".to_string(),
            );
        }
    }
}
