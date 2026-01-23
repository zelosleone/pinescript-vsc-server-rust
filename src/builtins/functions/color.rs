use crate::builtins::BuiltinFunction;
use crate::types::{BaseType, Type};

pub(super) fn list() -> Vec<BuiltinFunction> {
    vec![
        // Color helper functions
        BuiltinFunction {
            name: "color.from_gradient",
            signature: "color.from_gradient(value, bottom_value, top_value, bottom_color, top_color)",
            return_type: Type::scalar(BaseType::Color),
        },
        BuiltinFunction {
            name: "color.rgb",
            signature: "color.rgb(red, green, blue, transp?)",
            return_type: Type::scalar(BaseType::Color),
        },
        BuiltinFunction {
            name: "color.r",
            signature: "color.r(color)",
            return_type: Type::scalar(BaseType::Int),
        },
        BuiltinFunction {
            name: "color.g",
            signature: "color.g(color)",
            return_type: Type::scalar(BaseType::Int),
        },
        BuiltinFunction {
            name: "color.b",
            signature: "color.b(color)",
            return_type: Type::scalar(BaseType::Int),
        },
        BuiltinFunction {
            name: "color.t",
            signature: "color.t(color)",
            return_type: Type::scalar(BaseType::Int),
        },
    ]
}
