use crate::builtins::BuiltinFunction;
use crate::types::{BaseType, Type};

pub(super) fn list() -> Vec<BuiltinFunction> {
    vec![
        // Box getters
        BuiltinFunction {
            name: "box.get_top",
            signature: "box.get_top(id)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "box.get_bottom",
            signature: "box.get_bottom(id)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "box.get_left",
            signature: "box.get_left(id)",
            return_type: Type::scalar(BaseType::Int),
        },
        BuiltinFunction {
            name: "box.get_right",
            signature: "box.get_right(id)",
            return_type: Type::scalar(BaseType::Int),
        },
        BuiltinFunction {
            name: "box.delete",
            signature: "box.delete(id)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "box.copy",
            signature: "box.copy(id)",
            return_type: Type::unknown(),
        },
    ]
}
