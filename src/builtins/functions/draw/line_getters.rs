use crate::builtins::BuiltinFunction;
use crate::types::{BaseType, Type};

pub(super) fn list() -> Vec<BuiltinFunction> {
    vec![
        // Line getters
        BuiltinFunction {
            name: "line.get_x1",
            signature: "line.get_x1(id)",
            return_type: Type::scalar(BaseType::Int),
        },
        BuiltinFunction {
            name: "line.get_y1",
            signature: "line.get_y1(id)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "line.get_x2",
            signature: "line.get_x2(id)",
            return_type: Type::scalar(BaseType::Int),
        },
        BuiltinFunction {
            name: "line.get_y2",
            signature: "line.get_y2(id)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "line.get_price",
            signature: "line.get_price(id, x)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "line.delete",
            signature: "line.delete(id)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "line.copy",
            signature: "line.copy(id)",
            return_type: Type::unknown(),
        },
    ]
}
