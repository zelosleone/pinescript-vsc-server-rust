use tower_lsp::lsp_types::{DiagnosticSeverity, Range};
use tree_sitter::Node;

use crate::analysis::{Analyzer, ConditionContext};

impl<'a> Analyzer<'a> {
    pub(in crate::analysis) fn update_strategy_settings(
        &mut self,
        name: &str,
        args: &[(Option<String>, Node)],
        call_range: Range,
    ) {
        if name != "strategy" {
            return;
        }
        self.has_strategy_declaration = true;
        self.strategy_decl_range = Some(call_range);
        if let Some(pyramiding) = Self::find_call_arg_by_name(args, "pyramiding")
            && let Some(value) = self.eval_numeric_constant(pyramiding)
            && value >= 0.0
        {
            self.strategy_pyramiding = Some(value.round() as i64);
        }

        if let Some(initial_capital) = Self::find_call_arg_by_name(args, "initial_capital")
            && let Some(value) = self.eval_numeric_constant(initial_capital)
        {
            if value > 0.0 {
                self.strategy_initial_capital = Some(value);
                self.strategy_initial_capital_range = Some(self.range_for_node(initial_capital));
            } else if self.settings.enable_strategy_position_size_validation {
                self.push_diagnostic(
                    self.range_for_node(initial_capital),
                    DiagnosticSeverity::HINT,
                    "initial_capital should be > 0".to_string(),
                    "logic".to_string(),
                );
            }
        }

        if let Some(default_qty_type) = Self::find_call_arg_by_name(args, "default_qty_type") {
            let value = if default_qty_type.kind() == "attribute" {
                self.attribute_chain_name(default_qty_type)
            } else if default_qty_type.kind() == "identifier" {
                Some(self.node_text(default_qty_type))
            } else {
                self.eval_string_constant(default_qty_type)
            };
            self.strategy_default_qty_type = value;
            self.strategy_default_qty_type_range = Some(self.range_for_node(default_qty_type));
        }

        if let Some(default_qty_value) = Self::find_call_arg_by_name(args, "default_qty_value")
            && let Some(value) = self.eval_numeric_constant(default_qty_value)
        {
            self.strategy_default_qty_value = Some(value);
            self.strategy_default_qty_value_range = Some(self.range_for_node(default_qty_value));
        }

        self.check_strategy_default_qty_value();

        if !self.settings.enable_strategy_commission_slippage_validation {
            return;
        }

        if let Some(node) = Self::find_call_arg_by_name(args, "commission_value")
            && let Some(value) = self.eval_numeric_constant(node)
        {
            if value < 0.0 {
                self.push_diagnostic(
                    self.range_for_node(node),
                    DiagnosticSeverity::HINT,
                    "commission_value should be >= 0".to_string(),
                    "logic".to_string(),
                );
            } else if value > self.settings.commission_warn_threshold {
                self.push_diagnostic(
                    self.range_for_node(node),
                    DiagnosticSeverity::HINT,
                    "commission_value is unusually high".to_string(),
                    "logic".to_string(),
                );
            }
        }

        if let Some(node) = Self::find_call_arg_by_name(args, "slippage")
            && let Some(value) = self.eval_numeric_constant(node)
        {
            if value < 0.0 {
                self.push_diagnostic(
                    self.range_for_node(node),
                    DiagnosticSeverity::HINT,
                    "slippage should be >= 0".to_string(),
                    "logic".to_string(),
                );
            } else if value > self.settings.slippage_warn_threshold {
                self.push_diagnostic(
                    self.range_for_node(node),
                    DiagnosticSeverity::HINT,
                    "slippage is unusually large".to_string(),
                    "logic".to_string(),
                );
            }
        }
    }

    pub(in crate::analysis) fn check_strategy_default_qty_value(&mut self) {
        if !self.settings.enable_strategy_position_size_validation {
            return;
        }
        let Some(value) = self.strategy_default_qty_value else {
            return;
        };
        let Some(range) = self.strategy_default_qty_value_range else {
            return;
        };

        if value <= 0.0 {
            self.push_diagnostic(
                range,
                DiagnosticSeverity::HINT,
                "default_qty_value should be > 0".to_string(),
                "logic".to_string(),
            );
            return;
        }

        let qty_type = self.strategy_default_qty_type.as_deref().unwrap_or("");
        if qty_type == "strategy.percent_of_equity" {
            if value > self.settings.strategy_position_percent_max {
                self.push_diagnostic(
                    range,
                    DiagnosticSeverity::HINT,
                    "default_qty_value exceeds percent-of-equity limit".to_string(),
                    "logic".to_string(),
                );
            }
            return;
        }

        if qty_type == "strategy.cash"
            && let Some(initial_capital) = self.strategy_initial_capital
            && value > initial_capital
        {
            self.push_diagnostic(
                range,
                DiagnosticSeverity::HINT,
                "default_qty_value exceeds initial_capital".to_string(),
                "logic".to_string(),
            );
        }
    }

    pub(in crate::analysis) fn record_strategy_order_id(
        &mut self,
        name: &str,
        args: &[(Option<String>, Node)],
        call_range: Range,
    ) {
        if name != "strategy.entry" && name != "strategy.order" {
            return;
        }
        let Some((id_node, _idx)) = Self::find_call_arg(args, "id", 0) else {
            return;
        };
        let Some(id_value) = self.eval_string_constant(id_node) else {
            return;
        };
        if let Some(previous) = self.strategy_order_ids.get(&id_value).cloned() {
            self.push_diagnostic(
                self.range_for_node(id_node),
                DiagnosticSeverity::HINT,
                format!("Duplicate strategy order id `{}`", id_value),
                "logic".to_string(),
            );
            self.push_diagnostic(
                previous,
                DiagnosticSeverity::INFORMATION,
                format!("Previous `{}` order id usage here", id_value),
                "logic".to_string(),
            );
        } else {
            self.strategy_order_ids
                .insert(id_value.clone(), self.range_for_node(id_node));
        }
        self.strategy_entry_ids.insert(id_value);
        self.strategy_entry_count += 1;

        if self.strategy_entry_count > 1
            && self.strategy_pyramiding.unwrap_or(0) == 0
            && !self.reported_pyramiding_hint
        {
            let range = self.strategy_decl_range.unwrap_or(call_range);
            self.push_diagnostic(
                range,
                DiagnosticSeverity::HINT,
                "Multiple strategy entries without pyramiding configuration".to_string(),
                "logic".to_string(),
            );
            self.reported_pyramiding_hint = true;
        }
    }

    pub(in crate::analysis) fn check_strategy_direction_conflict(
        &mut self,
        name: &str,
        args: &[(Option<String>, Node)],
        call_range: Range,
    ) {
        if !self.settings.enable_strategy_direction_conflict {
            return;
        }
        if name != "strategy.entry" && name != "strategy.order" {
            return;
        }
        let direction_node = Self::find_call_arg(args, "direction", 1)
            .map(|(node, _)| node)
            .or_else(|| Self::find_call_arg_by_name(args, "direction"));
        let Some(direction_node) = direction_node else {
            return;
        };
        let direction = if direction_node.kind() == "attribute" {
            self.attribute_chain_name(direction_node)
        } else if direction_node.kind() == "identifier" {
            Some(self.node_text(direction_node))
        } else {
            None
        };
        let Some(direction) = direction else {
            return;
        };
        if direction == "strategy.long" || direction == "strategy.short" {
            self.strategy_entry_directions.insert(direction.clone());
        } else {
            return;
        }
        if self.reported_direction_conflict {
            return;
        }
        if self.strategy_entry_directions.contains("strategy.long")
            && self.strategy_entry_directions.contains("strategy.short")
            && self.strategy_pyramiding.unwrap_or(0) == 0
        {
            let range = self.strategy_decl_range.unwrap_or(call_range);
            self.push_diagnostic(
                range,
                DiagnosticSeverity::HINT,
                "Strategy uses both long and short entries without pyramiding configuration"
                    .to_string(),
                "logic".to_string(),
            );
            self.reported_direction_conflict = true;
        }
    }

    pub(in crate::analysis) fn check_strategy_oca_conflicts(
        &mut self,
        name: &str,
        args: &[(Option<String>, Node)],
    ) {
        if name != "strategy.entry" && name != "strategy.order" {
            return;
        }
        if !self.settings.enable_strategy_oca_conflicts {
            return;
        }
        let Some(oca_name_node) = Self::find_call_arg_by_name(args, "oca_name")
            .or_else(|| Self::find_call_arg(args, "oca_name", 5).map(|(node, _)| node))
        else {
            return;
        };
        let Some(oca_name) = self.eval_string_constant(oca_name_node) else {
            return;
        };
        let oca_type_node = Self::find_call_arg_by_name(args, "oca_type")
            .or_else(|| Self::find_call_arg(args, "oca_type", 6).map(|(node, _)| node));
        let Some(oca_type_node) = oca_type_node else {
            return;
        };
        let oca_type = if oca_type_node.kind() == "attribute" {
            self.attribute_chain_name(oca_type_node)
        } else if oca_type_node.kind() == "identifier" {
            Some(self.node_text(oca_type_node))
        } else {
            self.eval_string_constant(oca_type_node)
        };
        let Some(oca_type) = oca_type else {
            return;
        };

        if let Some((prev_type, prev_range)) = self.oca_groups.get(&oca_name).cloned() {
            if prev_type != oca_type {
                self.push_diagnostic(
                    self.range_for_node(oca_name_node),
                    DiagnosticSeverity::HINT,
                    format!(
                        "OCA group `{}` uses multiple types (`{}` vs `{}`)",
                        oca_name, prev_type, oca_type
                    ),
                    "logic".to_string(),
                );
                self.push_diagnostic(
                    prev_range,
                    DiagnosticSeverity::INFORMATION,
                    format!("Previous `{}` OCA type here", prev_type),
                    "logic".to_string(),
                );
            }
        } else {
            self.oca_groups
                .insert(oca_name, (oca_type, self.range_for_node(oca_name_node)));
        }
    }

    pub(in crate::analysis) fn check_strategy_quantity(
        &mut self,
        name: &str,
        args: &[(Option<String>, Node)],
    ) {
        let mut qty_nodes = Vec::new();
        match name {
            "strategy.entry" | "strategy.order" => {
                if let Some((node, _idx)) = Self::find_call_arg(args, "qty", 2) {
                    qty_nodes.push(node);
                }
            }
            "strategy.exit" => {
                if let Some((node, _idx)) = Self::find_call_arg(args, "qty", 2) {
                    qty_nodes.push(node);
                }
                if let Some((node, _idx)) = Self::find_call_arg(args, "qty_percent", 3) {
                    qty_nodes.push(node);
                }
            }
            _ => {}
        }

        for node in qty_nodes {
            let Some(value) = self.eval_numeric_constant(node) else {
                continue;
            };
            if value <= 0.0 {
                self.push_diagnostic(
                    self.range_for_node(node),
                    DiagnosticSeverity::HINT,
                    "Strategy quantity should be > 0".to_string(),
                    "logic".to_string(),
                );
            }
        }
    }

    pub(in crate::analysis) fn check_strategy_position_size(
        &mut self,
        name: &str,
        args: &[(Option<String>, Node)],
    ) {
        if !self.settings.enable_strategy_position_size_validation {
            return;
        }

        if name == "strategy.exit" {
            if let Some((node, _idx)) = Self::find_call_arg(args, "qty_percent", 3)
                && let Some(value) = self.eval_numeric_constant(node)
                && value > self.settings.strategy_position_percent_max
            {
                self.push_diagnostic(
                    self.range_for_node(node),
                    DiagnosticSeverity::HINT,
                    "Strategy qty_percent exceeds percent-of-equity limit".to_string(),
                    "logic".to_string(),
                );
            }
            return;
        }

        if name != "strategy.entry" && name != "strategy.order" {
            return;
        }
        let Some((qty_node, _idx)) = Self::find_call_arg(args, "qty", 2) else {
            return;
        };
        let Some(qty) = self.eval_numeric_constant(qty_node) else {
            return;
        };
        if qty <= 0.0 {
            return;
        }
        if self.strategy_default_qty_type.as_deref() != Some("strategy.cash") {
            return;
        }
        let Some(initial_capital) = self.strategy_initial_capital else {
            return;
        };
        if qty > initial_capital {
            self.push_diagnostic(
                self.range_for_node(qty_node),
                DiagnosticSeverity::HINT,
                "Strategy quantity exceeds initial_capital for cash sizing".to_string(),
                "logic".to_string(),
            );
        }
    }

    pub(in crate::analysis) fn check_strategy_alert_message(
        &mut self,
        name: &str,
        args: &[(Option<String>, Node)],
    ) {
        if !self.settings.enable_strategy_alert_message_validation {
            return;
        }

        let alert_node = match name {
            "strategy.entry" | "strategy.order" => Self::find_call_arg(args, "alert_message", 8),
            "strategy.exit" => Self::find_call_arg(args, "alert_message", 16),
            "strategy.close" => Self::find_call_arg(args, "alert_message", 2),
            "strategy.close_all" => Self::find_call_arg(args, "alert_message", 1),
            _ => None,
        };

        let Some((alert_node, _idx)) = alert_node else {
            return;
        };
        let Some(message) = self.eval_string_constant(alert_node) else {
            return;
        };
        if message.trim().is_empty() {
            self.push_diagnostic(
                self.range_for_node(alert_node),
                DiagnosticSeverity::HINT,
                "alert_message is empty".to_string(),
                "logic".to_string(),
            );
            return;
        }
        let max_len = self.settings.strategy_alert_message_max_length;
        if max_len > 0 && message.chars().count() as i64 > max_len {
            self.push_diagnostic(
                self.range_for_node(alert_node),
                DiagnosticSeverity::HINT,
                "alert_message is unusually long".to_string(),
                "logic".to_string(),
            );
        }
    }

    pub(in crate::analysis) fn check_strategy_price_bounds(
        &mut self,
        name: &str,
        args: &[(Option<String>, Node)],
    ) {
        if !self.settings.enable_strategy_price_validation {
            return;
        }
        let mut nodes = Vec::new();
        match name {
            "strategy.entry" | "strategy.order" => {
                if let Some((node, _idx)) = Self::find_call_arg(args, "limit", 3) {
                    nodes.push(node);
                }
                if let Some((node, _idx)) = Self::find_call_arg(args, "stop", 4) {
                    nodes.push(node);
                }
            }
            "strategy.exit" => {
                if let Some((node, _idx)) = Self::find_call_arg(args, "limit", 5) {
                    nodes.push(node);
                }
                if let Some((node, _idx)) = Self::find_call_arg(args, "stop", 7) {
                    nodes.push(node);
                }
            }
            _ => {}
        }

        for node in nodes {
            if let Some(value) = self.eval_numeric_constant(node)
                && value <= 0.0
            {
                self.push_diagnostic(
                    self.range_for_node(node),
                    DiagnosticSeverity::HINT,
                    "Strategy price should be > 0".to_string(),
                    "logic".to_string(),
                );
            }
        }
    }

    pub(in crate::analysis) fn check_strategy_limit_stop_relation(
        &mut self,
        name: &str,
        args: &[(Option<String>, Node)],
    ) {
        if name != "strategy.entry" && name != "strategy.order" {
            return;
        }
        if !self.settings.enable_strategy_limit_stop_validation {
            return;
        }
        let direction_node = Self::find_call_arg(args, "direction", 1)
            .map(|(node, _)| node)
            .or_else(|| Self::find_call_arg_by_name(args, "direction"));
        let Some(direction_node) = direction_node else {
            return;
        };
        let direction = if direction_node.kind() == "attribute" {
            self.attribute_chain_name(direction_node)
        } else if direction_node.kind() == "identifier" {
            Some(self.node_text(direction_node))
        } else {
            None
        };
        let Some(direction) = direction else {
            return;
        };
        let Some((limit_node, _idx)) = Self::find_call_arg(args, "limit", 3) else {
            return;
        };
        let Some((stop_node, _idx)) = Self::find_call_arg(args, "stop", 4) else {
            return;
        };
        let Some(limit) = self.eval_numeric_constant(limit_node) else {
            return;
        };
        let Some(stop) = self.eval_numeric_constant(stop_node) else {
            return;
        };
        if direction == "strategy.long" && limit < stop {
            self.push_diagnostic(
                self.range_for_node(limit_node),
                DiagnosticSeverity::HINT,
                "Long entry limit is below stop; check limit/stop ordering".to_string(),
                "logic".to_string(),
            );
        } else if direction == "strategy.short" && limit > stop {
            self.push_diagnostic(
                self.range_for_node(limit_node),
                DiagnosticSeverity::HINT,
                "Short entry limit is above stop; check limit/stop ordering".to_string(),
                "logic".to_string(),
            );
        }
    }

    pub(in crate::analysis) fn check_strategy_exit_profit_loss(
        &mut self,
        name: &str,
        args: &[(Option<String>, Node)],
    ) {
        if name != "strategy.exit" {
            return;
        }
        if !self.settings.enable_strategy_exit_profit_loss_validation {
            return;
        }
        let profit = Self::find_call_arg(args, "profit", 4)
            .and_then(|(node, _)| self.eval_numeric_constant(node).map(|value| (node, value)));
        let loss = Self::find_call_arg(args, "loss", 6)
            .and_then(|(node, _)| self.eval_numeric_constant(node).map(|value| (node, value)));
        if let Some((node, value)) = profit
            && value <= 0.0
        {
            self.push_diagnostic(
                self.range_for_node(node),
                DiagnosticSeverity::HINT,
                "strategy.exit profit should be > 0".to_string(),
                "logic".to_string(),
            );
        }
        if let Some((node, value)) = loss
            && value >= 0.0
        {
            self.push_diagnostic(
                self.range_for_node(node),
                DiagnosticSeverity::HINT,
                "strategy.exit loss should be < 0".to_string(),
                "logic".to_string(),
            );
        }
    }

    pub(in crate::analysis) fn check_strategy_trail(
        &mut self,
        name: &str,
        args: &[(Option<String>, Node)],
    ) {
        if name != "strategy.exit" {
            return;
        }
        if !self.settings.enable_strategy_trailing_validation {
            return;
        }
        let trail_price = Self::find_call_arg(args, "trail_price", 8);
        let trail_points = Self::find_call_arg(args, "trail_points", 9);
        let trail_offset = Self::find_call_arg(args, "trail_offset", 10);

        if let Some((trail_price_node, _)) = trail_price
            && trail_points.is_some()
        {
            self.push_diagnostic(
                self.range_for_node(trail_price_node),
                DiagnosticSeverity::HINT,
                "Specify either trail_price or trail_points, not both".to_string(),
                "logic".to_string(),
            );
        }
        if let Some((trail_offset_node, _)) = trail_offset
            && trail_price.is_none()
            && trail_points.is_none()
        {
            self.push_diagnostic(
                self.range_for_node(trail_offset_node),
                DiagnosticSeverity::HINT,
                "trail_offset requires trail_price or trail_points".to_string(),
                "logic".to_string(),
            );
        }
        for (label, node) in [
            ("trail_price", trail_price),
            ("trail_points", trail_points),
            ("trail_offset", trail_offset),
        ] {
            if let Some((node, _idx)) = node
                && let Some(value) = self.eval_numeric_constant(node)
                && value <= 0.0
            {
                self.push_diagnostic(
                    self.range_for_node(node),
                    DiagnosticSeverity::HINT,
                    format!("{} should be > 0", label),
                    "logic".to_string(),
                );
            }
        }
    }

    pub(in crate::analysis) fn check_strategy_position_management(
        &mut self,
        name: &str,
        args: &[(Option<String>, Node)],
    ) {
        if name == "strategy.close" {
            let Some((id_node, _idx)) = Self::find_call_arg(args, "id", 0) else {
                return;
            };
            let Some(id_value) = self.eval_string_constant(id_node) else {
                return;
            };
            if !self.strategy_entry_ids.contains(&id_value) {
                self.push_diagnostic(
                    self.range_for_node(id_node),
                    DiagnosticSeverity::HINT,
                    format!(
                        "strategy.close references `{}` without a matching entry id",
                        id_value
                    ),
                    "logic".to_string(),
                );
            }
            return;
        }

        if name == "strategy.exit" {
            let from_entry = Self::find_call_arg_by_name(args, "from_entry")
                .or_else(|| Self::find_call_arg(args, "from_entry", 1).map(|(node, _)| node));
            let Some(from_entry) = from_entry else {
                return;
            };
            let Some(entry_value) = self.eval_string_constant(from_entry) else {
                return;
            };
            if !self.strategy_entry_ids.contains(&entry_value) {
                self.push_diagnostic(
                    self.range_for_node(from_entry),
                    DiagnosticSeverity::HINT,
                    format!(
                        "strategy.exit references `{}` without a matching entry id",
                        entry_value
                    ),
                    "logic".to_string(),
                );
            }
        }
    }

    pub(in crate::analysis) fn check_closedtrades_index(
        &mut self,
        name: &str,
        args: &[(Option<String>, Node)],
    ) {
        if !name.starts_with("strategy.closedtrades.") {
            return;
        }
        let Some((idx_node, _idx)) = Self::find_call_arg(args, "trade_num", 0) else {
            return;
        };
        let Some(value) = self.eval_numeric_constant(idx_node) else {
            return;
        };
        if value < 0.0 {
            self.push_diagnostic(
                self.range_for_node(idx_node),
                DiagnosticSeverity::HINT,
                "strategy.closedtrades index is negative".to_string(),
                "logic".to_string(),
            );
        } else if value >= 1000.0 {
            self.push_diagnostic(
                self.range_for_node(idx_node),
                DiagnosticSeverity::HINT,
                "strategy.closedtrades index may exceed available trades".to_string(),
                "logic".to_string(),
            );
        }
    }

    pub(in crate::analysis) fn check_strategy_call_logic(
        &mut self,
        name: &str,
        args: &[(Option<String>, Node)],
        call_range: Range,
    ) {
        let (is_entry, is_exit) = match name {
            "strategy.entry" | "strategy.order" => (true, false),
            "strategy.exit" | "strategy.close" => (false, true),
            _ => (false, false),
        };
        if !is_entry && !is_exit {
            return;
        }

        let when_node = Self::find_call_arg_by_name(args, "when");
        if let Some(when_node) = when_node {
            if self.expr_has_close_zero(when_node) {
                self.push_diagnostic(
                    self.range_for_node(when_node),
                    DiagnosticSeverity::HINT,
                    "Strategy condition uses unshifted `close`; consider `close[1]` for confirmed bars".to_string(),
                    "logic".to_string(),
                );
            }
            if self.expr_has_barstate_flag(when_node) {
                self.push_diagnostic(
                    self.range_for_node(when_node),
                    DiagnosticSeverity::HINT,
                    "Verify barstate.isconfirmed/islast usage; behavior differs on historical bars"
                        .to_string(),
                    "logic".to_string(),
                );
            }
            return;
        }

        if self.condition_stack.last().is_some() {
            return;
        }

        self.push_diagnostic(
            call_range,
            DiagnosticSeverity::HINT,
            "Strategy call executes every bar; verify condition".to_string(),
            "logic".to_string(),
        );
    }

    pub(in crate::analysis) fn finalize_condition_context(&mut self, context: ConditionContext) {
        if context.strategy_entry && context.strategy_exit {
            self.push_diagnostic(
                context.range,
                DiagnosticSeverity::HINT,
                "Strategy entry and exit triggered in the same condition; may execute on the same bar"
                    .to_string(),
                "logic".to_string(),
            );
        }
        if (context.strategy_entry || context.strategy_exit) && context.condition_has_close_zero {
            self.push_diagnostic(
                context.range,
                DiagnosticSeverity::HINT,
                "Strategy condition uses unshifted `close`; consider `close[1]` for confirmed bars"
                    .to_string(),
                "logic".to_string(),
            );
        }
    }
}
