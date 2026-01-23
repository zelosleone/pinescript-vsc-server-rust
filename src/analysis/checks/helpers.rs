use tower_lsp::lsp_types::DiagnosticSeverity;
use tree_sitter::Node;

use crate::analysis::Analyzer;
use crate::types::{BaseType, Type};

#[derive(Clone, Debug)]
enum ConstValue {
    Bool(bool),
    Number(f64),
    String(String),
}

impl<'a> Analyzer<'a> {
    pub(in crate::analysis) fn is_direct_close_zero(&self, node: Node) -> bool {
        if node.kind() == "identifier" {
            return self.node_text(node) == "close";
        }
        if node.kind() == "subscript" {
            let series = node.child_by_field_name("series");
            let offset = node.child_by_field_name("offset");
            if let (Some(series), Some(offset)) = (series, offset)
                && series.kind() == "identifier"
                && self.node_text(series) == "close"
                && let Some(value) = self.eval_numeric_constant(offset)
            {
                return value == 0.0;
            }
        }
        false
    }

    pub(in crate::analysis) fn extract_identifier_name(&self, node: Node) -> Option<String> {
        if node.kind() == "identifier" {
            return Some(self.node_text(node));
        }
        if node.kind() == "attribute" {
            return self.attribute_chain_name(node);
        }
        None
    }

    pub(in crate::analysis) fn base_type_from_name(&self, name: &str) -> Option<BaseType> {
        match name {
            "int" => Some(BaseType::Int),
            "float" => Some(BaseType::Float),
            "bool" => Some(BaseType::Bool),
            "string" => Some(BaseType::String),
            "color" => Some(BaseType::Color),
            _ => None,
        }
    }

    pub(in crate::analysis) fn array_base_type_from_call(
        &self,
        call_name: &str,
    ) -> Option<BaseType> {
        match call_name {
            "array.new_int" => Some(BaseType::Int),
            "array.new_float" => Some(BaseType::Float),
            "array.new_bool" => Some(BaseType::Bool),
            "array.new_string" => Some(BaseType::String),
            "array.new_color" => Some(BaseType::Color),
            _ => None,
        }
    }

    pub(in crate::analysis) fn is_base_type_compatible(
        &self,
        expected: &BaseType,
        actual: &BaseType,
    ) -> bool {
        if expected == actual {
            return true;
        }
        matches!((expected, actual), (&BaseType::Float, &BaseType::Int))
    }

    pub(in crate::analysis) fn expr_is_zero(&self, node: Node) -> bool {
        if let Some(value) = self.eval_numeric_constant(node) {
            return value == 0.0;
        }
        if node.kind() == "math_operation" {
            let operator = node.child_by_field_name("operator");
            let left = node.child_by_field_name("left");
            let right = node.child_by_field_name("right");
            if let (Some(operator), Some(left), Some(right)) = (operator, left, right)
                && self.node_text(operator) == "-"
            {
                let left_text = self.normalized_node_text(left);
                let right_text = self.normalized_node_text(right);
                return !left_text.is_empty() && left_text == right_text;
            }
        }
        false
    }

    pub(in crate::analysis) fn expr_contains_na(&self, node: Node) -> bool {
        if node.kind() == "identifier" && self.node_text(node) == "na" {
            return true;
        }
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.is_named() && self.expr_contains_na(child) {
                return true;
            }
        }
        false
    }

    pub(in crate::analysis) fn eval_string_constant(&self, node: Node) -> Option<String> {
        if node.kind() != "string" {
            return None;
        }
        let value = self.node_text(node);
        if value.len() >= 2
            && ((value.starts_with('"') && value.ends_with('"'))
                || (value.starts_with('\'') && value.ends_with('\'')))
        {
            return Some(value[1..value.len() - 1].to_string());
        }
        Some(value)
    }

    pub(in crate::analysis) fn normalized_node_text(&self, node: Node) -> String {
        self.text
            .get(node.byte_range())
            .unwrap_or("")
            .split_whitespace()
            .collect::<String>()
    }

    pub(in crate::analysis) fn expr_has_close_zero(&self, node: Node) -> bool {
        match node.kind() {
            "subscript" => {
                let series = node.child_by_field_name("series");
                let offset = node.child_by_field_name("offset");
                if let (Some(series), Some(offset)) = (series, offset)
                    && series.kind() == "identifier"
                    && self.node_text(series) == "close"
                    && let Some(value) = self.eval_numeric_constant(offset)
                {
                    return value == 0.0;
                }
                return false;
            }
            "identifier" => {
                if self.node_text(node) == "close" {
                    return true;
                }
            }
            _ => {}
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.is_named() && self.expr_has_close_zero(child) {
                return true;
            }
        }
        false
    }

    pub(in crate::analysis) fn expr_has_barstate_flag(&self, node: Node) -> bool {
        if node.kind() == "attribute"
            && let Some(name) = self.attribute_chain_name(node)
            && (name == "barstate.isconfirmed" || name == "barstate.islast")
        {
            return true;
        }
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.is_named() && self.expr_has_barstate_flag(child) {
                return true;
            }
        }
        false
    }

    pub(in crate::analysis) fn find_call_arg_by_name<'b>(
        args: &[(Option<String>, Node<'b>)],
        param_name: &str,
    ) -> Option<Node<'b>> {
        for (opt_name, node) in args {
            if let Some(name) = opt_name
                && name == param_name
            {
                return Some(*node);
            }
        }
        None
    }

    pub(in crate::analysis) fn find_call_arg<'b>(
        args: &[(Option<String>, Node<'b>)],
        param_name: &str,
        pos: usize,
    ) -> Option<(Node<'b>, usize)> {
        for (idx, (opt_name, node)) in args.iter().enumerate() {
            if let Some(name) = opt_name
                && name == param_name
            {
                return Some((*node, idx));
            }
        }
        if args.len() > pos {
            return Some((args[pos].1, pos));
        }
        None
    }

    pub(in crate::analysis) fn check_call(
        &mut self,
        name: &str,
        args: &[(Option<String>, Node)],
        arg_types: &[Type],
    ) {
        let arg_nodes: Vec<Node> = args.iter().map(|(_, n)| *n).collect();
        let find_arg = |param_name: &str, pos: usize| -> Option<(Node, Type, usize)> {
            for (i, (opt_name, node)) in args.iter().enumerate() {
                if let Some(n) = opt_name
                    && n == param_name
                {
                    return Some((*node, arg_types[i].clone(), i));
                }
            }
            if args.len() > pos {
                return Some((args[pos].1, arg_types[pos].clone(), pos));
            }
            None
        };

        match name {
            "ta.sma" | "ta.ema" | "ta.rsi" => {
                self.check_series_numeric_arg(name, &arg_nodes, arg_types, 0);
                self.check_positive_length(name, &arg_nodes, 1);
            }
            "ta.atr" => {
                self.check_positive_length(name, &arg_nodes, 0);
            }
            "ta.macd" => {
                self.check_series_numeric_arg(name, &arg_nodes, arg_types, 0);
                self.check_positive_length(name, &arg_nodes, 1);
                self.check_positive_length(name, &arg_nodes, 2);
                self.check_positive_length(name, &arg_nodes, 3);
            }
            "plot" | "plotchar" | "plotshape" => {
                if let Some((first, ty, _idx)) = find_arg("series", 0)
                    && !ty.is_series()
                {
                    self.push_diagnostic(
                        self.range_for_node(first),
                        DiagnosticSeverity::WARNING,
                        format!(
                            "`{}` expects a series value; constant or scalar detected",
                            name
                        ),
                        "logic".to_string(),
                    );
                }
            }
            _ => {}
        }

        if name == "plot"
            && let Some((first, _, _)) = find_arg("series", 0)
            && self.is_constant_expression(first)
        {
            self.push_diagnostic(
                self.range_for_node(first),
                DiagnosticSeverity::WARNING,
                "Plotting a constant value will not show price movement".to_string(),
                "logic".to_string(),
            );
        }

        if name == "plotshape"
            && let Some((text_node, text_ty, _idx)) = find_arg("text", 5)
            && (!self.is_constant_expression(text_node)
                || text_ty.base() != BaseType::String
                || text_ty.is_series())
        {
            self.push_diagnostic(
                self.range_for_node(text_node),
                DiagnosticSeverity::ERROR,
                "`plotshape` 'text' parameter must be a constant string".to_string(),
                "validation".to_string(),
            );
        }

        if name == "plotchar"
            && let Some((char_node, char_ty, _idx)) = find_arg("char", 2)
        {
            if !self.is_constant_expression(char_node)
                || char_ty.base() != BaseType::String
                || char_ty.is_series()
            {
                self.push_diagnostic(
                    self.range_for_node(char_node),
                    DiagnosticSeverity::ERROR,
                    "`plotchar` 'char' parameter must be a constant string".to_string(),
                    "validation".to_string(),
                );
            } else if char_node.kind() == "string" {
                let txt = self.node_text(char_node);
                let inner = if txt.len() >= 2
                    && ((txt.starts_with('"') && txt.ends_with('"'))
                        || (txt.starts_with('\'') && txt.ends_with('\'')))
                {
                    txt[1..txt.len() - 1].to_string()
                } else {
                    txt.clone()
                };
                if inner.chars().count() != 1 {
                    self.push_diagnostic(
                        self.range_for_node(char_node),
                        DiagnosticSeverity::ERROR,
                        "`plotchar` 'char' parameter must be a single character constant string"
                            .to_string(),
                        "validation".to_string(),
                    );
                }
            }
        }
    }

    pub(in crate::analysis) fn check_series_numeric_arg(
        &mut self,
        name: &str,
        args: &[Node],
        arg_types: &[Type],
        idx: usize,
    ) {
        if let Some(arg_ty) = arg_types.get(idx) {
            let is_numeric = matches!(arg_ty.base(), BaseType::Float | BaseType::Int);
            if (!is_numeric || !arg_ty.is_series())
                && let Some(arg) = args.get(idx)
            {
                self.push_diagnostic(
                    self.range_for_node(*arg),
                    DiagnosticSeverity::ERROR,
                    format!("`{}` expects a numeric series input", name),
                    "validation".to_string(),
                );
            }
        }
    }

    pub(in crate::analysis) fn check_positive_length(
        &mut self,
        name: &str,
        args: &[Node],
        idx: usize,
    ) {
        if let Some(arg) = args.get(idx)
            && let Some(value) = self.eval_numeric_constant(*arg)
            && value <= 0.0
        {
            self.push_diagnostic(
                self.range_for_node(*arg),
                DiagnosticSeverity::ERROR,
                format!("`{}` length must be > 0", name),
                "runtime".to_string(),
            );
        }
    }

    pub(in crate::analysis) fn eval_numeric_constant(&self, node: Node) -> Option<f64> {
        match node.kind() {
            "integer" | "float" => self
                .text
                .get(node.byte_range())
                .and_then(|value| value.parse::<f64>().ok()),
            "unary_operation" => {
                let operator = node.child_by_field_name("operator")?;
                let value = node.child_by_field_name("argument")?;
                let sign = self.node_text(operator);
                let number = self.eval_numeric_constant(value)?;
                if sign == "-" {
                    Some(-number)
                } else {
                    Some(number)
                }
            }
            "math_operation" => {
                let left = node.child_by_field_name("left")?;
                let right = node.child_by_field_name("right")?;
                let operator = node.child_by_field_name("operator")?;
                let left_val = self.eval_numeric_constant(left)?;
                let right_val = self.eval_numeric_constant(right)?;
                let op = self.node_text(operator);
                match op.as_str() {
                    "+" => Some(left_val + right_val),
                    "-" => Some(left_val - right_val),
                    "*" => Some(left_val * right_val),
                    "/" => Some(left_val / right_val),
                    "%" => Some(left_val % right_val),
                    _ => None,
                }
            }
            "parenthesized_expression" => node
                .named_child(0)
                .and_then(|child| self.eval_numeric_constant(child)),
            _ => None,
        }
    }

    fn eval_const_value(&self, node: Node) -> Option<ConstValue> {
        match node.kind() {
            "true" => Some(ConstValue::Bool(true)),
            "false" => Some(ConstValue::Bool(false)),
            "identifier" => {
                let name = self.node_text(node);
                if name == "true" {
                    return Some(ConstValue::Bool(true));
                }
                if name == "false" {
                    return Some(ConstValue::Bool(false));
                }
                None
            }
            "string" => self.eval_string_constant(node).map(ConstValue::String),
            "parenthesized_expression" => node
                .named_child(0)
                .and_then(|child| self.eval_const_value(child)),
            _ => self.eval_numeric_constant(node).map(ConstValue::Number),
        }
    }

    pub(in crate::analysis) fn eval_bool_constant(&self, node: Node) -> Option<bool> {
        match node.kind() {
            "true" => return Some(true),
            "false" => return Some(false),
            "identifier" => {
                let name = self.node_text(node);
                if name == "true" {
                    return Some(true);
                }
                if name == "false" {
                    return Some(false);
                }
            }
            "parenthesized_expression" => {
                if let Some(child) = node.named_child(0) {
                    return self.eval_bool_constant(child);
                }
            }
            "unary_operation" => {
                let operator = node.child_by_field_name("operator")?;
                let argument = node.child_by_field_name("argument")?;
                if self.node_text(operator) == "not"
                    && let Some(value) = self.eval_bool_constant(argument)
                {
                    return Some(!value);
                }
            }
            "logical_operation" => {
                let left = node.child_by_field_name("left")?;
                let right = node.child_by_field_name("right")?;
                let operator = node.child_by_field_name("operator")?;
                let op = self.node_text(operator);
                let left_val = self.eval_bool_constant(left)?;
                let right_val = self.eval_bool_constant(right)?;
                return match op.as_str() {
                    "and" => Some(left_val && right_val),
                    "or" => Some(left_val || right_val),
                    _ => None,
                };
            }
            "comparison_operation" => {
                let left = node.child_by_field_name("left")?;
                let right = node.child_by_field_name("right")?;
                let operator = node.child_by_field_name("operator")?;
                let op = self.node_text(operator);
                let left_val = self.eval_const_value(left)?;
                let right_val = self.eval_const_value(right)?;
                return match (left_val, right_val) {
                    (ConstValue::Number(l), ConstValue::Number(r)) => match op.as_str() {
                        "==" => Some(l == r),
                        "!=" => Some(l != r),
                        "<" => Some(l < r),
                        "<=" => Some(l <= r),
                        ">" => Some(l > r),
                        ">=" => Some(l >= r),
                        _ => None,
                    },
                    (ConstValue::String(l), ConstValue::String(r)) => match op.as_str() {
                        "==" => Some(l == r),
                        "!=" => Some(l != r),
                        _ => None,
                    },
                    (ConstValue::Bool(l), ConstValue::Bool(r)) => match op.as_str() {
                        "==" => Some(l == r),
                        "!=" => Some(l != r),
                        _ => None,
                    },
                    _ => None,
                };
            }
            _ => {}
        }
        None
    }

    pub(in crate::analysis) fn is_constant_expression(&self, node: Node) -> bool {
        if let Some(value) = self.eval_numeric_constant(node) {
            return value.is_finite();
        }
        match node.kind() {
            "string" | "color" => true,
            "identifier" => {
                let name = self.node_text(node);
                self.builtins.value_type(&name).is_none() && name != "close"
            }
            _ => false,
        }
    }
}
