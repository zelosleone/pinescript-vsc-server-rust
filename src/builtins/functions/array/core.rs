use crate::builtins::BuiltinFunction;
use crate::types::{BaseType, Type};

pub(super) fn list() -> Vec<BuiltinFunction> {
    vec![
        // Array helpers
        BuiltinFunction {
            name: "array.new_int",
            signature: "array.new_int(size, val?)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "array.new_float",
            signature: "array.new_float(size, val?)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "array.new_bool",
            signature: "array.new_bool(size, val?)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "array.size",
            signature: "array.size(arr)",
            return_type: Type::scalar(BaseType::Int),
        },
        BuiltinFunction {
            name: "array.push",
            signature: "array.push(arr, value)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "array.pop",
            signature: "array.pop(arr)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "array.get",
            signature: "array.get(arr, index)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "array.set",
            signature: "array.set(arr, index, value)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "array.from",
            signature: "array.from(values...)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "array.clear",
            signature: "array.clear(arr)",
            return_type: Type::scalar(BaseType::Void),
        },
    ]
}
