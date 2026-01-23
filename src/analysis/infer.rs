use tree_sitter::Node;

use crate::analysis::Analyzer;
use crate::types::{BaseType, Type};

impl<'a> Analyzer<'a> {
    pub(super) fn infer_expr_type(&self, node: Node) -> Type {
        match node.kind() {
            "integer" => Type::scalar(BaseType::Int),
            "float" => Type::scalar(BaseType::Float),
            "string" | "double_quotted_string" | "single_quotted_string" => {
                Type::scalar(BaseType::String)
            }
            "color" => Type::scalar(BaseType::Color),
            "identifier" => {
                let name = self.node_text(node);
                if name == "true" || name == "false" {
                    return Type::scalar(BaseType::Bool);
                }
                if let Some(def_index) = self.resolve_symbol(&name) {
                    return self.definitions[def_index].ty.clone();
                }
                if let Some(builtin) = self.builtins.value_type(&name) {
                    return builtin.clone();
                }
                Type::unknown()
            }
            "attribute" => {
                if let Some(name) = self.attribute_chain_name(node) {
                    if let Some(ty) = self.builtins.value_type(&name) {
                        return ty.clone();
                    }
                    // If attribute matches an enum member (e.g. EnumName.member), return its enum type
                    if let Some(pos) = name.rfind('.') {
                        let object = &name[..pos];
                        let field = &name[pos + 1..];
                        if let Some(members) = self.enums.get(object)
                            && members.iter().any(|m| m == field)
                        {
                            return Type::UserDefined(object.to_string());
                        }
                    }
                }
                Type::unknown()
            }
            "parenthesized_expression" => node
                .named_child(0)
                .map(|child| self.infer_expr_type(child))
                .unwrap_or_else(Type::unknown),
            "math_operation" => self.infer_binary_math_type(node),
            "comparison_operation" | "logical_operation" => self.infer_comparison_type(node),
            "unary_operation" => self.infer_unary_type(node),
            "conditional_expression" => self.infer_conditional_type(node),
            "call" => self.infer_call_type(node),
            _ => Type::unknown(),
        }
    }

    pub(super) fn infer_call_type(&self, node: Node) -> Type {
        let Some(function_node) = node.child_by_field_name("function") else {
            return Type::unknown();
        };
        let call_name = self.call_name_from_node(function_node);
        if let Some(func) = self.builtins.function(&call_name) {
            return func.return_type.clone();
        }
        if let Some(def_index) = self.resolve_symbol(&call_name) {
            return self.definitions[def_index].ty.clone();
        }
        Type::unknown()
    }

    pub(super) fn infer_binary_math_type(&self, node: Node) -> Type {
        let left = node
            .child_by_field_name("left")
            .map(|n| self.infer_expr_type(n))
            .unwrap_or_else(Type::unknown);
        let right = node
            .child_by_field_name("right")
            .map(|n| self.infer_expr_type(n))
            .unwrap_or_else(Type::unknown);

        if left == Type::Unknown || right == Type::Unknown {
            return Type::unknown();
        }

        let is_series = left.is_series() || right.is_series();
        let base = match (left.base(), right.base()) {
            (BaseType::Float, _) | (_, BaseType::Float) => BaseType::Float,
            (BaseType::Int, BaseType::Int) => BaseType::Int,
            _ => BaseType::Unknown,
        };
        if is_series {
            Type::series(base)
        } else {
            Type::scalar(base)
        }
    }

    pub(super) fn infer_comparison_type(&self, node: Node) -> Type {
        let left = node
            .child_by_field_name("left")
            .map(|n| self.infer_expr_type(n))
            .unwrap_or_else(Type::unknown);
        let right = node
            .child_by_field_name("right")
            .map(|n| self.infer_expr_type(n))
            .unwrap_or_else(Type::unknown);
        let is_series = left.is_series() || right.is_series();
        if is_series {
            Type::series(BaseType::Bool)
        } else {
            Type::scalar(BaseType::Bool)
        }
    }

    pub(super) fn infer_unary_type(&self, node: Node) -> Type {
        node.child_by_field_name("argument")
            .map(|n| self.infer_expr_type(n))
            .unwrap_or_else(Type::unknown)
    }

    pub(super) fn infer_conditional_type(&self, node: Node) -> Type {
        let true_value = node
            .child_by_field_name("if_branch")
            .map(|n| self.infer_expr_type(n))
            .unwrap_or_else(Type::unknown);
        let false_value = node
            .child_by_field_name("else_branch")
            .map(|n| self.infer_expr_type(n))
            .unwrap_or_else(Type::unknown);
        if true_value == false_value {
            true_value
        } else {
            Type::unknown()
        }
    }

    pub(super) fn infer_return_type(&self, node: Node) -> Type {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "statement" || child.kind() == "block" {
                return self.infer_expr_type(child);
            }
        }
        Type::unknown()
    }

    pub(super) fn parse_declared_type(&self, node: Node) -> Type {
        let qualifier = node
            .child_by_field_name("qualifier")
            .map(|q| self.node_text(q))
            .unwrap_or_default();
        let type_node = node.child_by_field_name("type");
        let base = type_node
            .map(|n| self.node_text(n))
            .and_then(|name| match name.as_str() {
                "int" => Some(BaseType::Int),
                "float" => Some(BaseType::Float),
                "bool" => Some(BaseType::Bool),
                "string" => Some(BaseType::String),
                "color" => Some(BaseType::Color),
                _ => None,
            })
            .unwrap_or(BaseType::Unknown);
        if base == BaseType::Unknown {
            // If not a builtin base type, it could be a user-defined type (type/enum)
            if let Some(type_node) = type_node {
                let name = self.node_text(type_node);
                if let Some(def_index) = self.resolve_symbol(&name) {
                    let def = &self.definitions[def_index];
                    // If this symbol is a user-defined type, return a user type
                    if def.kind == crate::analysis::SymbolKind::Type {
                        return Type::UserDefined(name);
                    }
                }
            }
            return Type::unknown();
        }
        if qualifier.contains("series") {
            Type::series(base)
        } else {
            Type::scalar(base)
        }
    }
}
