use crate::builtins::BuiltinFunction;
use crate::types::{BaseType, Type};

pub(super) fn list() -> Vec<BuiltinFunction> {
    vec![
        // Array mutation functions
        BuiltinFunction {
            name: "array.insert",
            signature: "array.insert(arr, index, value)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "array.shift",
            signature: "array.shift(arr)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "array.unshift",
            signature: "array.unshift(arr, value)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "array.remove",
            signature: "array.remove(arr, index)",
            return_type: Type::unknown(),
        },
    ]
}
