use std::collections::{HashMap, HashSet};

use serde::Deserialize;
use tower_lsp::lsp_types::{Diagnostic, DocumentSymbol, Range, SymbolKind as LspSymbolKind};
use tree_sitter::Tree;

use crate::builtins::Builtins;
use crate::text::LineIndex;
use crate::types::{BaseType, Type};

mod checks;
mod infer;
mod symbols;
mod utils;
mod walk;

#[derive(Clone, Debug, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct AnalysisSettings {
    pub enable_shadowing_warnings: bool,
    pub enable_input_range_validation: bool,
    pub enable_table_bounds_checks: bool,
    pub enable_map_checks: bool,
    pub enable_chart_point_validation: bool,
    pub enable_session_validation: bool,
    pub enable_draw_object_lifecycle_checks: bool,
    pub enable_strategy_direction_conflict: bool,
    pub enable_constant_condition_warnings: bool,
    pub enable_ta_param_relationships: bool,
    pub enable_pivot_validation: bool,
    pub enable_strategy_limit_stop_validation: bool,
    pub enable_strategy_oca_conflicts: bool,
    pub enable_strategy_exit_profit_loss_validation: bool,
    pub enable_strategy_trailing_validation: bool,
    pub enable_duplicate_plot_titles: bool,
    pub enable_plot_style_consistency: bool,
    pub enable_draw_object_id_reuse: bool,
    pub enable_request_security_format_validation: bool,
    pub enable_request_security_expression_validation: bool,
    pub enable_color_value_validation: bool,
    pub enable_array_empty_checks: bool,
    pub enable_array_type_checks: bool,
    pub enable_string_na_checks: bool,
    pub enable_ta_length_consistency: bool,
    pub enable_strategy_commission_slippage_validation: bool,
    pub enable_strategy_price_validation: bool,
    pub enable_strategy_position_size_validation: bool,
    pub enable_strategy_alert_message_validation: bool,
    pub enable_function_recursion_detection: bool,
    pub enable_timeframe_format_validation: bool,
    pub array_unknown_index_warn_threshold: i64,
    pub loop_large_threshold: i64,
    pub history_reference_warn_threshold: i64,
    pub chart_point_index_warn_threshold: i64,
    pub ta_length_spread_ratio: i64,
    pub ta_length_spread_delta: i64,
    pub commission_warn_threshold: f64,
    pub slippage_warn_threshold: f64,
    pub strategy_position_percent_max: f64,
    pub strategy_alert_message_max_length: i64,
}

impl Default for AnalysisSettings {
    fn default() -> Self {
        Self {
            enable_shadowing_warnings: true,
            enable_input_range_validation: true,
            enable_table_bounds_checks: true,
            enable_map_checks: true,
            enable_chart_point_validation: true,
            enable_session_validation: true,
            enable_draw_object_lifecycle_checks: true,
            enable_strategy_direction_conflict: true,
            enable_constant_condition_warnings: true,
            enable_ta_param_relationships: true,
            enable_pivot_validation: true,
            enable_strategy_limit_stop_validation: true,
            enable_strategy_oca_conflicts: true,
            enable_strategy_exit_profit_loss_validation: true,
            enable_strategy_trailing_validation: true,
            enable_duplicate_plot_titles: true,
            enable_plot_style_consistency: true,
            enable_draw_object_id_reuse: true,
            enable_request_security_format_validation: true,
            enable_request_security_expression_validation: true,
            enable_color_value_validation: true,
            enable_array_empty_checks: true,
            enable_array_type_checks: true,
            enable_string_na_checks: true,
            enable_ta_length_consistency: true,
            enable_strategy_commission_slippage_validation: true,
            enable_strategy_price_validation: true,
            enable_strategy_position_size_validation: true,
            enable_strategy_alert_message_validation: true,
            enable_function_recursion_detection: true,
            enable_timeframe_format_validation: true,
            array_unknown_index_warn_threshold: 1000,
            loop_large_threshold: 100000,
            history_reference_warn_threshold: 5000,
            chart_point_index_warn_threshold: 5000,
            ta_length_spread_ratio: 10,
            ta_length_spread_delta: 50,
            commission_warn_threshold: 100.0,
            slippage_warn_threshold: 1000.0,
            strategy_position_percent_max: 100.0,
            strategy_alert_message_max_length: 500,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SymbolDef {
    pub name: String,
    pub kind: SymbolKind,
    pub range: Range,
    pub selection_range: Range,
    pub ty: Type,
    pub scope_depth: usize,
    pub is_builtin: bool,
}

#[derive(Clone, Debug)]
pub struct SymbolRef {
    pub name: String,
    pub range: Range,
    pub def_index: Option<usize>,
}

#[derive(Clone, Debug)]
pub struct AnalysisResult {
    pub diagnostics: Vec<Diagnostic>,
    pub definitions: Vec<SymbolDef>,
    pub references: Vec<SymbolRef>,
    pub document_symbols: Vec<DocumentSymbol>,
    pub enums: HashMap<String, Vec<String>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SymbolKind {
    Variable,
    Function,
    Parameter,
    LoopVariable,
    Builtin,
    /// User-defined type (type/enum)
    Type,
}

impl SymbolKind {
    pub fn to_lsp(self) -> LspSymbolKind {
        match self {
            SymbolKind::Variable => LspSymbolKind::VARIABLE,
            SymbolKind::Function => LspSymbolKind::FUNCTION,
            SymbolKind::Parameter => LspSymbolKind::VARIABLE,
            SymbolKind::LoopVariable => LspSymbolKind::VARIABLE,
            SymbolKind::Builtin => LspSymbolKind::CONSTANT,
            SymbolKind::Type => LspSymbolKind::CLASS,
        }
    }
}

pub(crate) struct Scope {
    symbols: HashMap<String, usize>,
}

#[derive(Clone, Debug)]
pub(crate) struct ConditionContext {
    range: Range,
    strategy_entry: bool,
    strategy_exit: bool,
    condition_has_close_zero: bool,
}

pub fn analyze_with_settings(
    text: &str,
    line_index: &LineIndex,
    tree: &Tree,
    settings: &AnalysisSettings,
) -> AnalysisResult {
    Analyzer::new(text, line_index, tree, settings.clone()).analyze()
}

pub(crate) struct Analyzer<'a> {
    text: &'a str,
    line_index: &'a LineIndex,
    tree: &'a Tree,
    builtins: Builtins,
    diagnostics: Vec<Diagnostic>,
    definitions: Vec<SymbolDef>,
    references: Vec<SymbolRef>,
    document_symbols: Vec<DocumentSymbol>,
    scopes: Vec<Scope>,
    duplicate_exprs: HashMap<String, Range>,
    has_indicator_or_strategy: bool,
    max_bars_back: Option<i64>,
    condition_stack: Vec<ConditionContext>,
    array_sizes: HashMap<String, i64>,
    array_types: HashMap<String, BaseType>,
    matrix_sizes: HashMap<String, (i64, i64)>,
    matrix_types: HashMap<String, BaseType>,
    table_sizes: HashMap<String, (i64, i64)>,
    map_known_empty: HashMap<String, bool>,
    map_key_types: HashMap<String, BaseType>,
    map_value_types: HashMap<String, BaseType>,
    strategy_order_ids: HashMap<String, Range>,
    strategy_entry_ids: HashSet<String>,
    strategy_entry_directions: HashSet<String>,
    strategy_entry_count: usize,
    strategy_pyramiding: Option<i64>,
    strategy_decl_range: Option<Range>,
    strategy_initial_capital: Option<f64>,
    strategy_initial_capital_range: Option<Range>,
    strategy_default_qty_type: Option<String>,
    strategy_default_qty_type_range: Option<Range>,
    strategy_default_qty_value: Option<f64>,
    strategy_default_qty_value_range: Option<Range>,
    has_strategy_declaration: bool,
    has_indicator_declaration: bool,
    has_barstate_isconfirmed: bool,
    reported_repainting_hint: bool,
    reported_pyramiding_hint: bool,
    reported_direction_conflict: bool,
    plot_titles: HashMap<String, Range>,
    plot_styles: HashMap<String, Range>,
    draw_object_vars: HashMap<String, (String, Range)>,
    deleted_draw_objects: HashMap<String, (String, Range)>,
    oca_groups: HashMap<String, (String, Range)>,
    ta_length_stats: HashMap<String, (i64, Range, i64, Range)>,
    function_stack: Vec<String>,
    function_calls: HashMap<String, HashSet<String>>,
    settings: AnalysisSettings,
    // user-defined enums found in the text: name -> member names
    enums: std::collections::HashMap<String, Vec<String>>,
    enum_member_ranges: std::collections::HashMap<String, HashMap<String, Range>>,
}

impl<'a> Analyzer<'a> {
    fn new(
        text: &'a str,
        line_index: &'a LineIndex,
        tree: &'a Tree,
        settings: AnalysisSettings,
    ) -> Self {
        let builtins = Builtins::new();
        let mut analyzer = Self {
            text,
            line_index,
            tree,
            builtins,
            diagnostics: Vec::new(),
            definitions: Vec::new(),
            references: Vec::new(),
            document_symbols: Vec::new(),
            scopes: Vec::new(),
            duplicate_exprs: HashMap::new(),
            has_indicator_or_strategy: false,
            max_bars_back: None,
            condition_stack: Vec::new(),
            array_sizes: HashMap::new(),
            array_types: HashMap::new(),
            matrix_sizes: HashMap::new(),
            matrix_types: HashMap::new(),
            table_sizes: HashMap::new(),
            map_known_empty: HashMap::new(),
            map_key_types: HashMap::new(),
            map_value_types: HashMap::new(),
            strategy_order_ids: HashMap::new(),
            strategy_entry_ids: HashSet::new(),
            strategy_entry_directions: HashSet::new(),
            strategy_entry_count: 0,
            strategy_pyramiding: None,
            strategy_decl_range: None,
            strategy_initial_capital: None,
            strategy_initial_capital_range: None,
            strategy_default_qty_type: None,
            strategy_default_qty_type_range: None,
            strategy_default_qty_value: None,
            strategy_default_qty_value_range: None,
            has_strategy_declaration: false,
            has_indicator_declaration: false,
            has_barstate_isconfirmed: false,
            reported_repainting_hint: false,
            reported_pyramiding_hint: false,
            reported_direction_conflict: false,
            plot_titles: HashMap::new(),
            plot_styles: HashMap::new(),
            draw_object_vars: HashMap::new(),
            deleted_draw_objects: HashMap::new(),
            oca_groups: HashMap::new(),
            ta_length_stats: HashMap::new(),
            function_stack: Vec::new(),
            function_calls: HashMap::new(),
            settings,
            enums: std::collections::HashMap::new(),
            enum_member_ranges: std::collections::HashMap::new(),
        };

        analyzer.scopes.push(Scope {
            symbols: HashMap::new(),
        });

        // initial enum map
        analyzer.enums = std::collections::HashMap::new();
        analyzer.enum_member_ranges = std::collections::HashMap::new();

        analyzer.seed_builtins();
        analyzer
    }

    fn analyze(mut self) -> AnalysisResult {
        self.collect_syntax_errors(self.tree.root_node());
        // Collect enums declared textually (supports enum declarations until parser gets updated)
        self.collect_enum_declarations();
        self.walk(self.tree.root_node());
        self.check_version_directive();
        self.check_entrypoint();
        self.report_unused_symbols();
        self.report_use_before_definition();
        self.report_ta_length_inconsistency();
        self.build_document_symbols();
        self.report_recursive_functions();

        AnalysisResult {
            diagnostics: self.diagnostics,
            definitions: self.definitions,
            references: self.references,
            document_symbols: self.document_symbols,
            enums: self.enums,
        }
    }
}

impl SymbolDef {
    fn kind_name(&self) -> &'static str {
        match self.kind {
            SymbolKind::Variable => "variable",
            SymbolKind::Function => "function",
            SymbolKind::Parameter => "parameter",
            SymbolKind::LoopVariable => "loop variable",
            SymbolKind::Builtin => "builtin",
            SymbolKind::Type => "type",
        }
    }
}
