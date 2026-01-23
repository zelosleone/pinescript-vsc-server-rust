use crate::builtins::BuiltinFunction;
use crate::types::{BaseType, Type};

pub(super) fn list() -> Vec<BuiltinFunction> {
    vec![
        // Array ordering functions
        BuiltinFunction {
            name: "array.reverse",
            signature: "array.reverse(arr)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "array.sort",
            signature: "array.sort(arr, order)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "array.sort_indices",
            signature: "array.sort_indices(arr, order)",
            return_type: Type::unknown(),
        },
    ]
}
