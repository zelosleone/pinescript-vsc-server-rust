use tower_lsp::lsp_types::DiagnosticSeverity;
use tree_sitter::Node;

use crate::analysis::Analyzer;

impl<'a> Analyzer<'a> {
    pub(in crate::analysis) fn check_plot_title(
        &mut self,
        name: &str,
        args: &[(Option<String>, Node)],
    ) {
        if name != "plot" {
            return;
        }
        if !self.settings.enable_duplicate_plot_titles {
            return;
        }
        let title_node = Self::find_call_arg(args, "title", 1).map(|(node, _)| node);
        let Some(title_node) = title_node else {
            return;
        };
        let Some(title) = self.eval_string_constant(title_node) else {
            return;
        };
        if title.trim().is_empty() {
            return;
        }
        if let Some(previous) = self.plot_titles.get(&title).cloned() {
            self.push_diagnostic(
                self.range_for_node(title_node),
                DiagnosticSeverity::HINT,
                format!("Duplicate plot title `{}`", title),
                "logic".to_string(),
            );
            self.push_diagnostic(
                previous,
                DiagnosticSeverity::INFORMATION,
                format!("Previous plot title `{}` here", title),
                "logic".to_string(),
            );
        } else {
            self.plot_titles
                .insert(title, self.range_for_node(title_node));
        }
    }

    pub(in crate::analysis) fn check_plot_style_consistency(
        &mut self,
        name: &str,
        args: &[(Option<String>, Node)],
    ) {
        if name != "plot" {
            return;
        }
        if !self.settings.enable_plot_style_consistency {
            return;
        }
        let style_node = Self::find_call_arg(args, "style", 4).map(|(node, _)| node);
        let Some(style_node) = style_node else {
            return;
        };
        let style = if style_node.kind() == "attribute" {
            self.attribute_chain_name(style_node)
        } else if style_node.kind() == "identifier" {
            Some(self.node_text(style_node))
        } else {
            self.eval_string_constant(style_node)
        };
        let Some(style) = style else {
            return;
        };
        if style.trim().is_empty() {
            return;
        }
        if self.plot_styles.contains_key(&style) {
            return;
        }
        if let Some((prev_style, prev_range)) =
            self.plot_styles.iter().next().map(|(s, r)| (s.clone(), *r))
        {
            self.push_diagnostic(
                self.range_for_node(style_node),
                DiagnosticSeverity::HINT,
                format!(
                    "plot uses multiple styles (`{}` vs `{}`); consider consistent styling",
                    prev_style, style
                ),
                "logic".to_string(),
            );
            self.push_diagnostic(
                prev_range,
                DiagnosticSeverity::INFORMATION,
                format!("Previous plot style `{}` used here", prev_style),
                "logic".to_string(),
            );
        }
        self.plot_styles
            .insert(style, self.range_for_node(style_node));
    }

    pub(in crate::analysis) fn maybe_warn_repainting(
        &mut self,
        name: &str,
        args: &[(Option<String>, Node)],
    ) {
        if self.reported_repainting_hint {
            return;
        }
        if !self.has_indicator_declaration || self.has_strategy_declaration {
            return;
        }
        if self.has_barstate_isconfirmed {
            return;
        }
        if name != "plot" && name != "plotshape" && name != "plotchar" && name != "plotbar" {
            return;
        }
        let Some((series_node, _idx)) = Self::find_call_arg(args, "series", 0) else {
            return;
        };
        if self.is_direct_close_zero(series_node) {
            self.push_diagnostic(
                self.range_for_node(series_node),
                DiagnosticSeverity::HINT,
                "Potential repainting: unconfirmed data used without barstate.isconfirmed"
                    .to_string(),
                "logic".to_string(),
            );
            self.reported_repainting_hint = true;
        }
    }
}
