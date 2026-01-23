use crate::builtins::BuiltinFunction;
use crate::types::{BaseType, Type};

pub(super) fn list() -> Vec<BuiltinFunction> {
    vec![
        // Array search functions
        BuiltinFunction {
            name: "array.includes",
            signature: "array.includes(arr, value)",
            return_type: Type::scalar(BaseType::Bool),
        },
        BuiltinFunction {
            name: "array.indexof",
            signature: "array.indexof(arr, value)",
            return_type: Type::scalar(BaseType::Int),
        },
        BuiltinFunction {
            name: "array.lastindexof",
            signature: "array.lastindexof(arr, value)",
            return_type: Type::scalar(BaseType::Int),
        },
        BuiltinFunction {
            name: "array.binary_search",
            signature: "array.binary_search(arr, val)",
            return_type: Type::scalar(BaseType::Int),
        },
        BuiltinFunction {
            name: "array.binary_search_leftmost",
            signature: "array.binary_search_leftmost(arr, val)",
            return_type: Type::scalar(BaseType::Int),
        },
        BuiltinFunction {
            name: "array.binary_search_rightmost",
            signature: "array.binary_search_rightmost(arr, val)",
            return_type: Type::scalar(BaseType::Int),
        },
    ]
}
