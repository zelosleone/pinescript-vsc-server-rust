use crate::builtins::BuiltinFunction;
use crate::types::{BaseType, Type};

pub(super) fn list() -> Vec<BuiltinFunction> {
    vec![
        // Array copy/slice functions
        BuiltinFunction {
            name: "array.concat",
            signature: "array.concat(arr1, arr2)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "array.copy",
            signature: "array.copy(arr)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "array.slice",
            signature: "array.slice(arr, index_from, index_to)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "array.fill",
            signature: "array.fill(arr, value, index_from?, index_to?)",
            return_type: Type::scalar(BaseType::Void),
        },
    ]
}
