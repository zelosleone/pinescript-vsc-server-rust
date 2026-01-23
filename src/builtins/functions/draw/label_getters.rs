use crate::builtins::BuiltinFunction;
use crate::types::{BaseType, Type};

pub(super) fn list() -> Vec<BuiltinFunction> {
    vec![
        // Label getters
        BuiltinFunction {
            name: "label.get_x",
            signature: "label.get_x(id)",
            return_type: Type::scalar(BaseType::Int),
        },
        BuiltinFunction {
            name: "label.get_y",
            signature: "label.get_y(id)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "label.get_text",
            signature: "label.get_text(id)",
            return_type: Type::scalar(BaseType::String),
        },
        BuiltinFunction {
            name: "label.get_color",
            signature: "label.get_color(id)",
            return_type: Type::scalar(BaseType::Color),
        },
        BuiltinFunction {
            name: "label.get_text_color",
            signature: "label.get_text_color(id)",
            return_type: Type::scalar(BaseType::Color),
        },
        BuiltinFunction {
            name: "label.get_style",
            signature: "label.get_style(id)",
            return_type: Type::scalar(BaseType::String),
        },
        BuiltinFunction {
            name: "label.get_size",
            signature: "label.get_size(id)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "label.get_tooltip",
            signature: "label.get_tooltip(id)",
            return_type: Type::scalar(BaseType::String),
        },
        BuiltinFunction {
            name: "label.get_text_formatting",
            signature: "label.get_text_formatting(id)",
            return_type: Type::scalar(BaseType::String),
        },
        BuiltinFunction {
            name: "label.get_text_font_family",
            signature: "label.get_text_font_family(id)",
            return_type: Type::scalar(BaseType::String),
        },
        BuiltinFunction {
            name: "label.copy",
            signature: "label.copy(id)",
            return_type: Type::unknown(),
        },
    ]
}
