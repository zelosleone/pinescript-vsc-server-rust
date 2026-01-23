use anyhow::{Result, anyhow};
use tower_lsp::lsp_types::TextDocumentContentChangeEvent;
use tree_sitter::{Parser, Tree};

use crate::analysis::{AnalysisResult, AnalysisSettings, analyze_with_settings};
use crate::text::LineIndex;

#[derive(Clone, Debug)]
pub struct Document {
    pub text: String,
    pub line_index: LineIndex,
    pub tree: Tree,
    pub analysis: AnalysisResult,
    settings: AnalysisSettings,
}

impl Document {
    #[allow(dead_code)] // Used in tests and as a convenience method
    pub fn new(text: String) -> Result<Self> {
        Self::new_with_settings(text, AnalysisSettings::default())
    }

    pub fn new_with_settings(text: String, settings: AnalysisSettings) -> Result<Self> {
        let line_index = LineIndex::new(&text);
        let tree = parse_text(&text)?;
        let analysis = analyze_with_settings(&text, &line_index, &tree, &settings);
        Ok(Self {
            text,
            line_index,
            tree,
            analysis,
            settings,
        })
    }

    pub fn apply_changes(&mut self, changes: Vec<TextDocumentContentChangeEvent>) -> Result<()> {
        for change in changes {
            if let Some(range) = change.range {
                let start = self.line_index.position_to_offset(&self.text, range.start);
                let end = self.line_index.position_to_offset(&self.text, range.end);
                if start <= end && end <= self.text.len() {
                    self.text.replace_range(start..end, &change.text);
                }
                self.line_index = LineIndex::new(&self.text);
            } else {
                self.text = change.text;
                self.line_index = LineIndex::new(&self.text);
            }
        }

        self.tree = parse_text(&self.text)?;
        self.analysis =
            analyze_with_settings(&self.text, &self.line_index, &self.tree, &self.settings);
        Ok(())
    }

    pub fn update_settings(&mut self, settings: AnalysisSettings) -> Result<()> {
        self.settings = settings;
        self.tree = parse_text(&self.text)?;
        self.analysis =
            analyze_with_settings(&self.text, &self.line_index, &self.tree, &self.settings);
        Ok(())
    }
}

fn parse_text(text: &str) -> Result<Tree> {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_pine::language())
        .map_err(|_| anyhow!("failed to load Pine grammar"))?;
    parser
        .parse(text, None)
        .ok_or_else(|| anyhow!("failed to parse document"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tower_lsp::lsp_types::DiagnosticSeverity;

    fn has_message(diags: &[tower_lsp::lsp_types::Diagnostic], needle: &str) -> bool {
        diags.iter().any(|diag| diag.message.contains(needle))
    }

    fn has_error(diags: &[tower_lsp::lsp_types::Diagnostic], needle: &str) -> bool {
        diags.iter().any(|diag| {
            diag.severity == Some(DiagnosticSeverity::ERROR) && diag.message.contains(needle)
        })
    }

    #[test]
    fn valid_script_has_no_diagnostics() {
        let text = "//@version=6\nindicator(\"X\")\nplot(ta.sma(close, 14))\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(doc.analysis.diagnostics.is_empty());
    }

    #[test]
    fn missing_version_warns() {
        let text = "indicator(\"X\")\nplot(close)\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "Missing //@version=6 directive"
        ));
    }

    #[test]
    fn missing_entrypoint_warns() {
        let text = "//@version=6\nx = 1\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "Missing indicator() or strategy() declaration"
        ));
    }

    #[test]
    fn negative_length_errors() {
        let text = "//@version=6\nindicator(\"X\")\nplot(ta.sma(close, -1))\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_error(&doc.analysis.diagnostics, "length must be > 0"));
    }

    #[test]
    fn enum_duplicate_member_errors() {
        let text = "//@version=6\nenum Direction\n    up = \"Up\"\n    up = \"Still Up\"\n\nindicator(\"E\")\nplot(1)\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_error(
            &doc.analysis.diagnostics,
            "Duplicate enum member"
        ));
    }

    #[test]
    fn enum_invalid_title_errors() {
        let text = "//@version=6\nenum Direction\n    up = 1\n\nindicator(\"E\")\nplot(1)\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_error(&doc.analysis.diagnostics, "Invalid enum title"));
    }

    #[test]
    fn plotshape_text_must_be_const_string() {
        let text = "//@version=6\nindicator(\"X\")\nplotshape(true, text = str.tostring(close))\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_error(
            &doc.analysis.diagnostics,
            "must be a constant string"
        ));
    }

    #[test]
    fn plotchar_char_must_be_single_char() {
        let text = "//@version=6\nindicator(\"X\")\nplotchar(true, char = \"XX\")\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_error(
            &doc.analysis.diagnostics,
            "single character constant string"
        ));
    }

    #[test]
    fn plotchar_char_must_be_const() {
        let text = "//@version=6\nindicator(\"X\")\nplotchar(true, char = str.tostring(close))\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_error(
            &doc.analysis.diagnostics,
            "must be a constant string"
        ));
    }

    #[test]
    fn unused_function_warns() {
        let text = "//@version=6\nindicator(\"X\")\nmy_func() => 1\nplot(1)\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "Unused function `my_func`"
        ));
    }

    #[test]
    fn unused_enum_member_warns() {
        let text = "//@version=6\nenum Direction\n    up = \"Up\"\n    down = \"Down\"\n\nindicator(\"X\")\nplot(Direction.up == Direction.up ? 1 : 0)\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "Unused enum member `Direction.down`"
        ));
    }

    #[test]
    fn negative_history_reference_warns() {
        let text = "//@version=6\nindicator(\"X\")\nplot(close[-1])\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "Possible lookahead bias: negative history reference uses future bars"
        ));
    }

    #[test]
    fn request_security_missing_params_warns() {
        let text = "//@version=6\nindicator(\"X\")\nrequest.security(\"A\", \"D\", close)\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "request.security missing `lookahead`"
        ));
        assert!(has_message(
            &doc.analysis.diagnostics,
            "request.security missing `gaps` parameter"
        ));
    }

    #[test]
    fn request_security_lookahead_on_warns() {
        let text = "//@version=6\nindicator(\"X\")\nrequest.security(\"A\", \"D\", close, lookahead = barmerge.lookahead_on)\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "request.security lookahead set to barmerge.lookahead_on"
        ));
    }

    #[test]
    fn history_reference_exceeds_max_bars_back_warns() {
        let text = "//@version=6\nindicator(\"X\")\nmax_bars_back(close, 10)\nplot(close[20])\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "History reference 20 exceeds max_bars_back 10"
        ));
    }

    #[test]
    fn strategy_entry_exit_same_condition_warns() {
        let text = "//@version=6\nstrategy(\"X\")\nif close > open\n    strategy.entry(\"L\", strategy.long)\n    strategy.close(\"L\")\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "Strategy entry and exit triggered in the same condition"
        ));
    }

    #[test]
    fn strategy_condition_close_zero_warns() {
        let text = "//@version=6\nstrategy(\"X\")\nif close[0] > open\n    strategy.entry(\"L\", strategy.long)\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "Strategy condition uses unshifted `close`"
        ));
    }

    #[test]
    fn barstate_condition_warns() {
        let text = "//@version=6\nindicator(\"X\")\nif barstate.islast\n    plot(close)\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "Verify barstate.isconfirmed/islast usage"
        ));
    }

    #[test]
    fn strategy_call_without_condition_warns() {
        let text = "//@version=6\nstrategy(\"X\")\nstrategy.entry(\"L\", strategy.long)\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "Strategy call executes every bar; verify condition"
        ));
    }

    #[test]
    fn division_by_zero_warns() {
        let text = "//@version=6\nindicator(\"X\")\nplot(close / 0)\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "Division by zero; divisor evaluates to 0"
        ));
    }

    #[test]
    fn duplicate_strategy_order_id_warns() {
        let text = "//@version=6\nstrategy(\"X\")\nstrategy.entry(\"L\", strategy.long)\nstrategy.entry(\"L\", strategy.long)\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "Duplicate strategy order id `L`"
        ));
    }

    #[test]
    fn strategy_quantity_invalid_warns() {
        let text =
            "//@version=6\nstrategy(\"X\")\nstrategy.entry(\"L\", strategy.long, qty = -1)\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "Strategy quantity should be > 0"
        ));
    }

    #[test]
    fn array_index_oob_warns() {
        let text =
            "//@version=6\nindicator(\"X\")\narr = array.new_int(5, 0)\nplot(array.get(arr, 10))\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "Array index 10 is out of bounds for size 5"
        ));
    }

    #[test]
    fn matrix_index_oob_warns() {
        let text = "//@version=6\nindicator(\"X\")\nm = matrix.new<int>(2, 2)\nplot(matrix.get(m, 3, 0))\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "Matrix row index 3 is out of bounds for 2 rows"
        ));
    }

    #[test]
    fn loop_range_large_warns() {
        let text = "//@version=6\nindicator(\"X\")\nfor i = 0 to 1000000\n    x = i\nplot(1)\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "Loop range is very large; this may be slow"
        ));
    }

    #[test]
    fn while_true_warns() {
        let text = "//@version=6\nindicator(\"X\")\nwhile true\n    x = 1\nplot(1)\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "While loop condition is always true; ensure it terminates"
        ));
    }

    #[test]
    fn math_domain_log_zero_warns() {
        let text = "//@version=6\nindicator(\"X\")\nplot(math.log(0))\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "math.log input is 0; result is undefined"
        ));
    }

    #[test]
    fn strategy_close_without_entry_warns() {
        let text = "//@version=6\nstrategy(\"X\")\nstrategy.close(\"L\")\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "strategy.close references `L` without a matching entry id"
        ));
    }

    #[test]
    fn na_argument_warns() {
        let text = "//@version=6\nindicator(\"X\")\nplot(ta.sma(na, 14))\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "Argument may be na; guard with na() or nz()"
        ));
    }

    #[test]
    fn timeframe_param_empty_warns() {
        let text = "//@version=6\nindicator(\"X\")\nplot(timeframe.change(\"\"))\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "timeframe parameter is empty"
        ));
    }

    #[test]
    fn pyramiding_conflict_warns() {
        let text = "//@version=6\nstrategy(\"X\")\nstrategy.entry(\"L1\", strategy.long)\nstrategy.entry(\"L2\", strategy.long)\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "Multiple strategy entries without pyramiding configuration"
        ));
    }

    #[test]
    fn use_before_definition_warns() {
        let text = "//@version=6\nindicator(\"X\")\nplot(x)\nx = 1\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "Possible use before definition of `x`"
        ));
    }

    #[test]
    fn matrix_size_mismatch_warns() {
        let text = "//@version=6\nindicator(\"X\")\nm1 = matrix.new<int>(2, 3)\nm2 = matrix.new<int>(4, 3)\nplot(matrix.sum(m1, m2))\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "Matrix operation requires matching dimensions"
        ));
    }

    #[test]
    fn closedtrades_index_warns() {
        let text = "//@version=6\nstrategy(\"X\")\nplot(strategy.closedtrades.profit(-1))\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "strategy.closedtrades index is negative"
        ));
    }

    #[test]
    fn repainting_warning_in_indicator() {
        let text = "//@version=6\nindicator(\"X\")\nplot(close)\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "Potential repainting: unconfirmed data used without barstate.isconfirmed"
        ));
    }

    #[test]
    fn ta_param_relationships_warns() {
        let text = "//@version=6\nindicator(\"X\")\nta.stoch(10, 5, 14, 3)\nta.macd(close, 20, 10, 5)\nta.bb(close, 20, 0)\nta.sar(0.04, 0.02, 0.03)\nta.percentile_nearest_rank(close, 14, 120)\nta.dmi(10, 20)\nta.pivothigh(close, 0, 2)\nplot(1)\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "ta.stoch expects k < d"
        ));
        assert!(has_message(
            &doc.analysis.diagnostics,
            "ta.macd expects fastlen < slowlen"
        ));
        assert!(has_message(
            &doc.analysis.diagnostics,
            "Multiplier should be > 0"
        ));
        assert!(has_message(
            &doc.analysis.diagnostics,
            "ta.sar expects start < inc"
        ));
        assert!(has_message(
            &doc.analysis.diagnostics,
            "Percentile should be in [0, 100]"
        ));
        assert!(has_message(
            &doc.analysis.diagnostics,
            "ta.dmi expects lensig < length"
        ));
        assert!(has_message(
            &doc.analysis.diagnostics,
            "leftbars should be > 0"
        ));
    }

    #[test]
    fn strategy_param_checks_warns() {
        let text = "//@version=6\nstrategy(\"X\", commission_value = 200, slippage = 2000)\nstrategy.entry(\"L1\", strategy.long, limit = 10, stop = 20, oca_name = \"grp\", oca_type = strategy.oca.cancel)\nstrategy.order(\"L2\", strategy.long, oca_name = \"grp\", oca_type = strategy.oca.reduce)\nstrategy.exit(\"L3\", from_entry = \"L1\", profit = 0, loss = 1, trail_price = 1, trail_points = 2)\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "commission_value is unusually high"
        ));
        assert!(has_message(
            &doc.analysis.diagnostics,
            "slippage is unusually large"
        ));
        assert!(has_message(
            &doc.analysis.diagnostics,
            "Long entry limit is below stop"
        ));
        assert!(has_message(
            &doc.analysis.diagnostics,
            "OCA group `grp` uses multiple types"
        ));
        assert!(has_message(
            &doc.analysis.diagnostics,
            "strategy.exit profit should be > 0"
        ));
        assert!(has_message(
            &doc.analysis.diagnostics,
            "strategy.exit loss should be < 0"
        ));
        assert!(has_message(
            &doc.analysis.diagnostics,
            "Specify either trail_price or trail_points"
        ));
    }

    #[test]
    fn duplicate_plot_title_warns() {
        let text = "//@version=6\nindicator(\"X\")\nplot(close, title = \"A\")\nplot(open, title = \"A\")\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "Duplicate plot title `A`"
        ));
    }

    #[test]
    fn draw_object_id_reuse_warns() {
        let text = "//@version=6\nindicator(\"X\")\nlabel_id = label.new(bar_index, close)\nlabel_id := label.new(bar_index, close)\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "Drawing object id `label_id` reused for multiple label objects"
        ));
    }

    #[test]
    fn request_security_timeframe_invalid_warns() {
        let text = "//@version=6\nindicator(\"X\")\nrequest.security(\"A\", \"abc\", close)\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "Invalid timeframe format `abc`"
        ));
    }

    #[test]
    fn array_empty_ops_warns() {
        let text = "//@version=6\nindicator(\"X\")\narr = array.new_int(0, 0)\narray.pop(arr)\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "Array is empty; operation will fail"
        ));
    }

    #[test]
    fn string_na_warns() {
        let text = "//@version=6\nindicator(\"X\")\nplot(str.length(na))\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "String function argument may be na"
        ));
    }

    #[test]
    fn ta_length_spread_warns() {
        let text =
            "//@version=6\nindicator(\"X\")\nplot(ta.sma(close, 5))\nplot(ta.sma(close, 200))\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "Large ta.sma length spread"
        ));
    }

    #[test]
    fn strategy_price_bounds_warns() {
        let text = "//@version=6\nstrategy(\"X\")\nstrategy.entry(\"L\", strategy.long, limit = 0, stop = -1)\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "Strategy price should be > 0"
        ));
    }

    #[test]
    fn timeframe_invalid_format_warns() {
        let text = "//@version=6\nindicator(\"X\")\nplot(timeframe.change(\"abc\"))\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "Invalid timeframe format `abc`"
        ));
    }

    #[test]
    fn shadowing_warns() {
        let text = "//@version=6\nindicator(\"X\")\nx = 1\nif close > open\n    x = 2\nplot(x)\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "shadows an outer definition"
        ));
    }

    #[test]
    fn constant_condition_warns() {
        let text = "//@version=6\nindicator(\"X\")\nif 1 > 2\n    plot(1)\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "Condition is always false"
        ));
    }

    #[test]
    fn request_security_missing_expression_warns() {
        let text = "//@version=6\nindicator(\"X\")\nrequest.security(\"A\", \"D\")\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "request.security missing expression parameter"
        ));
    }

    #[test]
    fn color_value_out_of_range_warns() {
        let text = "//@version=6\nindicator(\"X\")\nplot(1, color = color.rgb(300, -1, 0, 200))\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(&doc.analysis.diagnostics, "out of range"));
    }

    #[test]
    fn plot_style_consistency_warns() {
        let text = "//@version=6\nindicator(\"X\")\nplot(close, style = plot.style_line)\nplot(open, style = plot.style_area)\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "plot uses multiple styles"
        ));
    }

    #[test]
    fn strategy_alert_message_warns() {
        let text = "//@version=6\nstrategy(\"X\")\nstrategy.entry(\"L\", strategy.long, alert_message = \"\")\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "alert_message is empty"
        ));
    }

    #[test]
    fn array_type_mismatch_warns() {
        let text = "//@version=6\nindicator(\"X\")\narr = array.new_int(1, 0)\narray.push(arr, \"x\")\nplot(1)\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(&doc.analysis.diagnostics, "Array value type"));
    }

    #[test]
    fn matrix_type_mismatch_warns() {
        let text = "//@version=6\nindicator(\"X\")\nm = matrix.new<int>(1, 1)\nmatrix.set(m, 0, 0, 1.5)\nplot(1)\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(&doc.analysis.diagnostics, "Matrix value type"));
    }

    #[test]
    fn recursive_function_warns() {
        let text = "//@version=6\nindicator(\"X\")\nmy_func() => my_func()\nplot(1)\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "Function `my_func` is recursive"
        ));
    }

    #[test]
    fn strategy_default_qty_value_warns() {
        let text = "//@version=6\nstrategy(\"X\", initial_capital = 1000, default_qty_type = strategy.cash, default_qty_value = 2000)\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "default_qty_value exceeds initial_capital"
        ));
    }

    #[test]
    fn input_range_validation_warns() {
        let text =
            "//@version=6\nindicator(\"X\")\ninput.int(100, minval = 50, maxval = 75)\nplot(1)\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "input defval exceeds maxval"
        ));
    }

    #[test]
    fn table_bounds_warns() {
        let text = "//@version=6\nindicator(\"X\")\nt = table.new(position.top_right, 2, 2)\ntable.cell(t, 3, 0, \"x\")\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(&doc.analysis.diagnostics, "Table column"));
    }

    #[test]
    fn map_get_empty_warns() {
        let text = "//@version=6\nindicator(\"X\")\nm = map.new()\nmap.get(m, \"x\")\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(&doc.analysis.diagnostics, "Map may be empty"));
    }

    #[test]
    fn chart_point_index_warns() {
        let text = "//@version=6\nindicator(\"X\")\npt = chart.point.from_index(-1)\nplot(1)\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "chart.point.from_index index should be >= 0"
        ));
    }

    #[test]
    fn session_string_warns() {
        let text = "//@version=6\nindicator(\"X\")\ninput.session(\"9999-9999\")\nplot(1)\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "Session string format is invalid"
        ));
    }

    #[test]
    fn deleted_draw_object_warns() {
        let text = "//@version=6\nindicator(\"X\")\nlabel_id = label.new(bar_index, close)\nlabel.delete(label_id)\nlabel.set_text(label_id, \"x\")\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "Use of deleted label id"
        ));
    }

    #[test]
    fn strategy_direction_conflict_warns() {
        let text = "//@version=6\nstrategy(\"X\")\nstrategy.entry(\"L\", strategy.long)\nstrategy.entry(\"S\", strategy.short)\n";
        let doc = Document::new(text.to_string()).unwrap();
        assert!(has_message(
            &doc.analysis.diagnostics,
            "Strategy uses both long and short entries"
        ));
    }
}
