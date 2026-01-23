use crate::builtins::BuiltinFunction;
use crate::types::{BaseType, Type};

pub(super) fn list() -> Vec<BuiltinFunction> {
    vec![
        // Map functions
        BuiltinFunction {
            name: "map.new",
            signature: "map.new()",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "map.put",
            signature: "map.put(m, key, value)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "map.get",
            signature: "map.get(m, key)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "map.contains",
            signature: "map.contains(m, key)",
            return_type: Type::scalar(BaseType::Bool),
        },
        BuiltinFunction {
            name: "map.keys",
            signature: "map.keys(m)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "map.values",
            signature: "map.values(m)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "map.remove",
            signature: "map.remove(m, key)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "map.clear",
            signature: "map.clear(m)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "map.put_all",
            signature: "map.put_all(m1, m2)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "map.copy",
            signature: "map.copy(m)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "map.size",
            signature: "map.size(m)",
            return_type: Type::scalar(BaseType::Int),
        },
    ]
}
