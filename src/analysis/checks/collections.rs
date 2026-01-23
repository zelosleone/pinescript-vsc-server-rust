use tower_lsp::lsp_types::{DiagnosticSeverity, Range};
use tree_sitter::Node;

use crate::analysis::Analyzer;
use crate::types::BaseType;

impl<'a> Analyzer<'a> {
    pub(in crate::analysis) fn update_collection_sizes(&mut self, var_name: &str, init_node: Node) {
        if init_node.kind() != "call" {
            return;
        }
        let Some(function_node) = init_node.child_by_field_name("function") else {
            return;
        };
        let call_name = self.call_name_from_node(function_node);
        let raw_args = Self::collect_call_arguments(init_node);
        let args: Vec<(Option<String>, Node)> = raw_args
            .into_iter()
            .map(|(opt_name_node, value)| (opt_name_node.map(|n| self.node_text(n)), value))
            .collect();

        if call_name == "array.from" {
            let mut types = Vec::new();
            for (_opt, node) in &args {
                let ty = self.infer_expr_type(*node).base();
                if ty != BaseType::Unknown {
                    types.push(ty);
                }
            }
            if let Some(first) = types.first().cloned()
                && types.iter().all(|ty| *ty == first)
            {
                self.array_types.insert(var_name.to_string(), first);
            }
            return;
        }

        if call_name.starts_with("array.new") {
            if let Some(base) = self.array_base_type_from_call(&call_name) {
                self.array_types.insert(var_name.to_string(), base);
            } else if call_name == "array.new"
                && let Some((value_node, _idx)) = Self::find_call_arg(&args, "initial_value", 1)
                    .or_else(|| Self::find_call_arg(&args, "val", 1))
            {
                let inferred = self.infer_expr_type(value_node);
                let base = inferred.base();
                if base != BaseType::Unknown {
                    self.array_types.insert(var_name.to_string(), base);
                }
            }

            let Some((size_node, _idx)) = Self::find_call_arg(&args, "size", 0) else {
                return;
            };
            let Some(value) = self.eval_numeric_constant(size_node) else {
                return;
            };
            if value < 0.0 {
                return;
            }
            self.array_sizes
                .insert(var_name.to_string(), value.round() as i64);
            return;
        }

        if call_name.starts_with("matrix.new") {
            if let Some(template_name) = self.template_type_name(function_node) {
                if let Some(base) = self.base_type_from_name(&template_name) {
                    self.matrix_types.insert(var_name.to_string(), base);
                }
            } else if let Some((value_node, _idx)) = Self::find_call_arg(&args, "initial_value", 2)
            {
                let inferred = self.infer_expr_type(value_node);
                let base = inferred.base();
                if base != BaseType::Unknown {
                    self.matrix_types.insert(var_name.to_string(), base);
                }
            }

            let Some((rows_node, _idx)) = Self::find_call_arg(&args, "rows", 0) else {
                return;
            };
            let Some((cols_node, _idx)) = Self::find_call_arg(&args, "columns", 1) else {
                return;
            };
            let Some(rows) = self.eval_numeric_constant(rows_node) else {
                return;
            };
            let Some(cols) = self.eval_numeric_constant(cols_node) else {
                return;
            };
            if rows <= 0.0 || cols <= 0.0 {
                return;
            }
            self.matrix_sizes.insert(
                var_name.to_string(),
                (rows.round() as i64, cols.round() as i64),
            );
            return;
        }

        if call_name == "table.new" {
            let rows_node = Self::find_call_arg(&args, "rows", 1)
                .or_else(|| Self::find_call_arg(&args, "row", 1))
                .map(|(node, _)| node);
            let cols_node = Self::find_call_arg(&args, "cols", 2)
                .or_else(|| Self::find_call_arg(&args, "columns", 2))
                .map(|(node, _)| node);
            let (Some(rows_node), Some(cols_node)) = (rows_node, cols_node) else {
                return;
            };
            let Some(rows) = self.eval_numeric_constant(rows_node) else {
                return;
            };
            let Some(cols) = self.eval_numeric_constant(cols_node) else {
                return;
            };
            if rows <= 0.0 || cols <= 0.0 {
                return;
            }
            self.table_sizes.insert(
                var_name.to_string(),
                (rows.round() as i64, cols.round() as i64),
            );
            return;
        }

        if call_name == "map.new" {
            self.map_known_empty.insert(var_name.to_string(), true);
            self.map_key_types.remove(var_name);
            self.map_value_types.remove(var_name);
        }
    }

    pub(in crate::analysis) fn check_array_bounds(
        &mut self,
        args: &[(Option<String>, Node)],
        index_pos: usize,
    ) {
        let Some((arr_node, _idx)) = Self::find_call_arg(args, "arr", 0) else {
            return;
        };
        let Some((index_node, _idx)) = Self::find_call_arg(args, "index", index_pos) else {
            return;
        };
        let Some(name) = self.extract_identifier_name(arr_node) else {
            return;
        };
        let Some(index_value) = self.eval_numeric_constant(index_node) else {
            return;
        };
        if index_value < 0.0 {
            self.push_diagnostic(
                self.range_for_node(index_node),
                DiagnosticSeverity::HINT,
                "Array index is negative; this will fail at runtime".to_string(),
                "logic".to_string(),
            );
            return;
        }
        let index_value = index_value.round() as i64;
        if let Some(size) = self.array_sizes.get(&name) {
            if *size == 0 && self.settings.enable_array_empty_checks {
                self.push_diagnostic(
                    self.range_for_node(index_node),
                    DiagnosticSeverity::HINT,
                    "Array is empty; index access will fail".to_string(),
                    "logic".to_string(),
                );
                return;
            }
            if index_value >= *size {
                self.push_diagnostic(
                    self.range_for_node(index_node),
                    DiagnosticSeverity::HINT,
                    format!(
                        "Array index {} is out of bounds for size {}",
                        index_value, size
                    ),
                    "logic".to_string(),
                );
            }
        } else if self.settings.array_unknown_index_warn_threshold > 0
            && index_value >= self.settings.array_unknown_index_warn_threshold
        {
            self.push_diagnostic(
                self.range_for_node(index_node),
                DiagnosticSeverity::HINT,
                format!(
                    "Array index {} may be out of bounds; array size unknown",
                    index_value
                ),
                "logic".to_string(),
            );
        }
    }

    pub(in crate::analysis) fn check_array_empty_ops(&mut self, args: &[(Option<String>, Node)]) {
        if !self.settings.enable_array_empty_checks {
            return;
        }
        let Some((arr_node, _idx)) = Self::find_call_arg(args, "arr", 0) else {
            return;
        };
        let Some(name) = self.extract_identifier_name(arr_node) else {
            return;
        };
        let Some(size) = self.array_sizes.get(&name) else {
            return;
        };
        if *size == 0 {
            self.push_diagnostic(
                self.range_for_node(arr_node),
                DiagnosticSeverity::HINT,
                "Array is empty; operation will fail".to_string(),
                "logic".to_string(),
            );
        }
    }

    pub(in crate::analysis) fn check_array_type_consistency(
        &mut self,
        call_name: &str,
        args: &[(Option<String>, Node)],
    ) {
        if !self.settings.enable_array_type_checks {
            return;
        }

        if call_name == "array.from" {
            let mut types = Vec::new();
            for (_opt, node) in args {
                let ty = self.infer_expr_type(*node).base();
                if ty != BaseType::Unknown {
                    types.push(ty);
                }
            }
            if let Some(first) = types.first().cloned()
                && types.iter().any(|ty| *ty != first)
            {
                self.push_diagnostic(
                    args.first()
                        .map(|(_, node)| self.range_for_node(*node))
                        .unwrap_or_default(),
                    DiagnosticSeverity::HINT,
                    "array.from uses mixed value types".to_string(),
                    "logic".to_string(),
                );
            }
            return;
        }

        let Some((arr_node, _idx)) = Self::find_call_arg(args, "arr", 0) else {
            return;
        };
        let Some(array_name) = self.extract_identifier_name(arr_node) else {
            return;
        };
        let expected = self.array_types.get(&array_name).cloned();
        let Some(expected) = expected else {
            return;
        };
        let value_node = match call_name {
            "array.push" => Self::find_call_arg(args, "value", 1),
            "array.set" => Self::find_call_arg(args, "value", 2),
            "array.insert" => Self::find_call_arg(args, "value", 2),
            "array.unshift" => Self::find_call_arg(args, "value", 1),
            _ => None,
        };
        let Some((value_node, _idx)) = value_node else {
            return;
        };
        let actual = self.infer_expr_type(value_node).base();
        if actual == BaseType::Unknown {
            return;
        }
        if !self.is_base_type_compatible(&expected, &actual) {
            self.push_diagnostic(
                self.range_for_node(value_node),
                DiagnosticSeverity::HINT,
                format!(
                    "Array value type `{}` does not match expected `{}`",
                    actual.display_name(),
                    expected.display_name()
                ),
                "logic".to_string(),
            );
        }
    }

    pub(in crate::analysis) fn check_map_operations(
        &mut self,
        name: &str,
        args: &[(Option<String>, Node)],
    ) {
        if !self.settings.enable_map_checks {
            return;
        }
        if !name.starts_with("map.") {
            return;
        }

        let Some((map_node, _idx)) = Self::find_call_arg(args, "m", 0) else {
            return;
        };
        let Some(map_name) = self.extract_identifier_name(map_node) else {
            return;
        };

        match name {
            "map.put" => {
                let key_node = Self::find_call_arg(args, "key", 1).map(|(node, _)| node);
                let value_node = Self::find_call_arg(args, "value", 2).map(|(node, _)| node);
                if let Some(key_node) = key_node {
                    let key_type = self.infer_expr_type(key_node).base();
                    if key_type != BaseType::Unknown {
                        match self.map_key_types.get(&map_name) {
                            Some(expected)
                                if !self.is_base_type_compatible(expected, &key_type) =>
                            {
                                self.push_diagnostic(
                                    self.range_for_node(key_node),
                                    DiagnosticSeverity::HINT,
                                    "map.put key type does not match existing map keys".to_string(),
                                    "logic".to_string(),
                                );
                            }
                            None => {
                                self.map_key_types.insert(map_name.clone(), key_type);
                            }
                            _ => {}
                        }
                    }
                }
                if let Some(value_node) = value_node {
                    let value_type = self.infer_expr_type(value_node).base();
                    if value_type != BaseType::Unknown {
                        match self.map_value_types.get(&map_name) {
                            Some(expected)
                                if !self.is_base_type_compatible(expected, &value_type) =>
                            {
                                self.push_diagnostic(
                                    self.range_for_node(value_node),
                                    DiagnosticSeverity::HINT,
                                    "map.put value type does not match existing map values"
                                        .to_string(),
                                    "logic".to_string(),
                                );
                            }
                            None => {
                                self.map_value_types.insert(map_name.clone(), value_type);
                            }
                            _ => {}
                        }
                    }
                }
                self.map_known_empty.insert(map_name, false);
            }
            "map.put_all" => {
                self.map_known_empty.insert(map_name, false);
            }
            "map.clear" => {
                self.map_known_empty.insert(map_name, true);
            }
            "map.get" | "map.remove" | "map.keys" | "map.values" => {
                if self.map_known_empty.get(&map_name) == Some(&true) {
                    self.push_diagnostic(
                        self.range_for_node(map_node),
                        DiagnosticSeverity::HINT,
                        "Map may be empty; operation could fail or return na".to_string(),
                        "logic".to_string(),
                    );
                }
                if name == "map.get"
                    && let Some((key_node, _idx)) = Self::find_call_arg(args, "key", 1)
                {
                    let key_type = self.infer_expr_type(key_node).base();
                    if let Some(expected) = self.map_key_types.get(&map_name)
                        && key_type != BaseType::Unknown
                        && !self.is_base_type_compatible(expected, &key_type)
                    {
                        self.push_diagnostic(
                            self.range_for_node(key_node),
                            DiagnosticSeverity::HINT,
                            "map.get key type does not match existing map keys".to_string(),
                            "logic".to_string(),
                        );
                    }
                }
            }
            _ => {}
        }
    }

    pub(in crate::analysis) fn check_matrix_bounds(
        &mut self,
        args: &[(Option<String>, Node)],
        row_pos: usize,
        col_pos: usize,
    ) {
        let Some((matrix_node, _idx)) = Self::find_call_arg(args, "m", 0) else {
            return;
        };
        let Some(name) = self.extract_identifier_name(matrix_node) else {
            return;
        };
        let row_node = Self::find_call_arg(args, "row", row_pos)
            .map(|(node, _)| node)
            .or_else(|| Self::find_call_arg(args, "index", row_pos).map(|(node, _)| node));
        let col_node = Self::find_call_arg(args, "column", col_pos)
            .map(|(node, _)| node)
            .or_else(|| Self::find_call_arg(args, "col", col_pos).map(|(node, _)| node));
        let (Some(row_node), Some(col_node)) = (row_node, col_node) else {
            return;
        };
        let row_value = self.eval_numeric_constant(row_node);
        let col_value = self.eval_numeric_constant(col_node);

        if let Some(row_value) = row_value
            && row_value < 0.0
        {
            self.push_diagnostic(
                self.range_for_node(row_node),
                DiagnosticSeverity::HINT,
                "Matrix row index is negative; this will fail at runtime".to_string(),
                "logic".to_string(),
            );
            return;
        }
        if let Some(col_value) = col_value
            && col_value < 0.0
        {
            self.push_diagnostic(
                self.range_for_node(col_node),
                DiagnosticSeverity::HINT,
                "Matrix column index is negative; this will fail at runtime".to_string(),
                "logic".to_string(),
            );
            return;
        }

        let Some((rows, cols)) = self.matrix_sizes.get(&name).cloned() else {
            return;
        };
        if let Some(row_value) = row_value
            && row_value.round() as i64 >= rows
        {
            self.push_diagnostic(
                self.range_for_node(row_node),
                DiagnosticSeverity::HINT,
                format!(
                    "Matrix row index {} is out of bounds for {} rows",
                    row_value.round() as i64,
                    rows
                ),
                "logic".to_string(),
            );
        }
        if let Some(col_value) = col_value
            && col_value.round() as i64 >= cols
        {
            self.push_diagnostic(
                self.range_for_node(col_node),
                DiagnosticSeverity::HINT,
                format!(
                    "Matrix column index {} is out of bounds for {} columns",
                    col_value.round() as i64,
                    cols
                ),
                "logic".to_string(),
            );
        }
    }

    pub(in crate::analysis) fn check_table_bounds(
        &mut self,
        name: &str,
        args: &[(Option<String>, Node)],
    ) {
        if !self.settings.enable_table_bounds_checks {
            return;
        }
        let Some((table_node, _idx)) = Self::find_call_arg(args, "id", 0) else {
            return;
        };
        let Some(table_name) = self.extract_identifier_name(table_node) else {
            return;
        };
        let Some((rows, cols)) = self.table_sizes.get(&table_name).cloned() else {
            return;
        };

        let mut col_row_pairs: Vec<(Node, Node)> = Vec::new();
        match name {
            "table.cell"
            | "table.cell_set_text"
            | "table.cell_set_bgcolor"
            | "table.cell_set_text_color"
            | "table.cell_set_text_halign"
            | "table.cell_set_text_valign"
            | "table.cell_set_text_size"
            | "table.cell_set_text_formatting"
            | "table.cell_set_text_font_family"
            | "table.cell_set_tooltip"
            | "table.cell_set_width"
            | "table.cell_set_height" => {
                let col = Self::find_call_arg(args, "column", 1).map(|(node, _)| node);
                let row = Self::find_call_arg(args, "row", 2).map(|(node, _)| node);
                if let (Some(col), Some(row)) = (col, row) {
                    col_row_pairs.push((col, row));
                }
            }
            "table.merge_cells" | "table.clear" => {
                let start_col = Self::find_call_arg(args, "start_column", 1)
                    .or_else(|| Self::find_call_arg(args, "start_col", 1))
                    .map(|(node, _)| node);
                let start_row = Self::find_call_arg(args, "start_row", 2).map(|(node, _)| node);
                let end_col = Self::find_call_arg(args, "end_column", 3)
                    .or_else(|| Self::find_call_arg(args, "end_col", 3))
                    .map(|(node, _)| node);
                let end_row = Self::find_call_arg(args, "end_row", 4).map(|(node, _)| node);
                if let (Some(start_col), Some(start_row)) = (start_col, start_row) {
                    col_row_pairs.push((start_col, start_row));
                }
                if let (Some(end_col), Some(end_row)) = (end_col, end_row) {
                    col_row_pairs.push((end_col, end_row));
                }
            }
            _ => {}
        }

        for (col_node, row_node) in col_row_pairs {
            if let Some(col) = self.eval_numeric_constant(col_node)
                && (col < 0.0 || col.round() as i64 >= cols)
            {
                self.push_diagnostic(
                    self.range_for_node(col_node),
                    DiagnosticSeverity::HINT,
                    format!("Table column {} is out of bounds for {} columns", col, cols),
                    "logic".to_string(),
                );
            }
            if let Some(row) = self.eval_numeric_constant(row_node)
                && (row < 0.0 || row.round() as i64 >= rows)
            {
                self.push_diagnostic(
                    self.range_for_node(row_node),
                    DiagnosticSeverity::HINT,
                    format!("Table row {} is out of bounds for {} rows", row, rows),
                    "logic".to_string(),
                );
            }
        }
    }

    pub(in crate::analysis) fn check_matrix_index_bounds(
        &mut self,
        args: &[(Option<String>, Node)],
        index_pos: usize,
        is_row: bool,
    ) {
        let Some((matrix_node, _idx)) = Self::find_call_arg(args, "m", 0) else {
            return;
        };
        let Some(name) = self.extract_identifier_name(matrix_node) else {
            return;
        };
        let Some((index_node, _idx)) = Self::find_call_arg(args, "index", index_pos) else {
            return;
        };
        let Some(value) = self.eval_numeric_constant(index_node) else {
            return;
        };
        if value < 0.0 {
            self.push_diagnostic(
                self.range_for_node(index_node),
                DiagnosticSeverity::HINT,
                "Matrix index is negative; this will fail at runtime".to_string(),
                "logic".to_string(),
            );
            return;
        }
        let Some((rows, cols)) = self.matrix_sizes.get(&name).cloned() else {
            return;
        };
        let limit = if is_row { rows } else { cols };
        let label = if is_row { "row" } else { "column" };
        let value = value.round() as i64;
        if value >= limit {
            self.push_diagnostic(
                self.range_for_node(index_node),
                DiagnosticSeverity::HINT,
                format!(
                    "Matrix {} index {} is out of bounds for {} {}",
                    label, value, limit, label
                ),
                "logic".to_string(),
            );
        }
    }

    pub(in crate::analysis) fn check_matrix_swap_bounds(
        &mut self,
        args: &[(Option<String>, Node)],
        first_pos: usize,
        second_pos: usize,
        is_row: bool,
    ) {
        let Some((matrix_node, _idx)) = Self::find_call_arg(args, "m", 0) else {
            return;
        };
        let Some(name) = self.extract_identifier_name(matrix_node) else {
            return;
        };
        let Some((first_node, _idx)) = Self::find_call_arg(args, "row1", first_pos)
            .or_else(|| Self::find_call_arg(args, "col1", first_pos))
        else {
            return;
        };
        let Some((second_node, _idx)) = Self::find_call_arg(args, "row2", second_pos)
            .or_else(|| Self::find_call_arg(args, "col2", second_pos))
        else {
            return;
        };
        let Some((rows, cols)) = self.matrix_sizes.get(&name).cloned() else {
            return;
        };
        let limit = if is_row { rows } else { cols };
        let label = if is_row { "row" } else { "column" };
        for node in [first_node, second_node] {
            if let Some(value) = self.eval_numeric_constant(node) {
                if value < 0.0 {
                    self.push_diagnostic(
                        self.range_for_node(node),
                        DiagnosticSeverity::HINT,
                        "Matrix index is negative; this will fail at runtime".to_string(),
                        "logic".to_string(),
                    );
                } else if value.round() as i64 >= limit {
                    self.push_diagnostic(
                        self.range_for_node(node),
                        DiagnosticSeverity::HINT,
                        format!(
                            "Matrix {} index {} is out of bounds for {} {}",
                            label,
                            value.round() as i64,
                            limit,
                            label
                        ),
                        "logic".to_string(),
                    );
                }
            }
        }
    }

    pub(in crate::analysis) fn check_matrix_size_compat(
        &mut self,
        name: &str,
        args: &[(Option<String>, Node)],
        call_range: Range,
    ) {
        let Some((m1_node, _idx)) = Self::find_call_arg(args, "m1", 0) else {
            return;
        };
        let Some((m2_node, _idx)) = Self::find_call_arg(args, "m2", 1) else {
            return;
        };
        let Some(m1_name) = self.extract_identifier_name(m1_node) else {
            return;
        };
        let Some(m2_name) = self.extract_identifier_name(m2_node) else {
            return;
        };
        let Some((m1_rows, m1_cols)) = self.matrix_sizes.get(&m1_name) else {
            return;
        };
        let Some((m2_rows, m2_cols)) = self.matrix_sizes.get(&m2_name) else {
            return;
        };

        match name {
            "matrix.sum" | "matrix.diff" => {
                if m1_rows != m2_rows || m1_cols != m2_cols {
                    self.push_diagnostic(
                        call_range,
                        DiagnosticSeverity::HINT,
                        "Matrix operation requires matching dimensions".to_string(),
                        "logic".to_string(),
                    );
                }
            }
            "matrix.mult" => {
                if m1_cols != m2_rows {
                    self.push_diagnostic(
                        call_range,
                        DiagnosticSeverity::HINT,
                        "Matrix multiplication requires m1 columns == m2 rows".to_string(),
                        "logic".to_string(),
                    );
                }
            }
            "matrix.concat" => {
                let dimension = Self::find_call_arg(args, "dimension", 2)
                    .and_then(|(node, _)| self.eval_numeric_constant(node))
                    .map(|value| value.round() as i64);
                if let Some(dimension) = dimension {
                    if dimension == 0 && m1_cols != m2_cols {
                        self.push_diagnostic(
                            call_range,
                            DiagnosticSeverity::HINT,
                            "Matrix concat by rows requires matching column counts".to_string(),
                            "logic".to_string(),
                        );
                    } else if dimension == 1 && m1_rows != m2_rows {
                        self.push_diagnostic(
                            call_range,
                            DiagnosticSeverity::HINT,
                            "Matrix concat by columns requires matching row counts".to_string(),
                            "logic".to_string(),
                        );
                    } else if dimension != 0 && dimension != 1 {
                        self.push_diagnostic(
                            call_range,
                            DiagnosticSeverity::HINT,
                            "Matrix concat dimension should be 0 (rows) or 1 (columns)".to_string(),
                            "logic".to_string(),
                        );
                    }
                }
            }
            _ => {}
        }
    }

    pub(in crate::analysis) fn check_matrix_type_consistency(
        &mut self,
        call_name: &str,
        args: &[(Option<String>, Node)],
    ) {
        if !self.settings.enable_array_type_checks {
            return;
        }
        if call_name != "matrix.set" && call_name != "matrix.fill" {
            return;
        }
        let Some((matrix_node, _idx)) = Self::find_call_arg(args, "m", 0) else {
            return;
        };
        let Some(name) = self.extract_identifier_name(matrix_node) else {
            return;
        };
        let Some(expected) = self.matrix_types.get(&name).cloned() else {
            return;
        };
        let value_node = match call_name {
            "matrix.set" => Self::find_call_arg(args, "value", 3),
            "matrix.fill" => Self::find_call_arg(args, "value", 1),
            _ => None,
        };
        let Some((value_node, _idx)) = value_node else {
            return;
        };
        let actual = self.infer_expr_type(value_node).base();
        if actual == BaseType::Unknown {
            return;
        }
        if !self.is_base_type_compatible(&expected, &actual) {
            self.push_diagnostic(
                self.range_for_node(value_node),
                DiagnosticSeverity::HINT,
                format!(
                    "Matrix value type `{}` does not match expected `{}`",
                    actual.display_name(),
                    expected.display_name()
                ),
                "logic".to_string(),
            );
        }
    }
}
