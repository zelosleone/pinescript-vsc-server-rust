use crate::builtins::BuiltinFunction;
use crate::types::{BaseType, Type};

pub(super) fn list() -> Vec<BuiltinFunction> {
    vec![
        // Label setters
        BuiltinFunction {
            name: "label.set_x",
            signature: "label.set_x(id, x)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "label.set_y",
            signature: "label.set_y(id, y)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "label.set_xy",
            signature: "label.set_xy(id, x, y)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "label.set_point",
            signature: "label.set_point(id, point)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "label.set_text",
            signature: "label.set_text(id, text)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "label.set_color",
            signature: "label.set_color(id, color)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "label.set_textcolor",
            signature: "label.set_textcolor(id, textcolor)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "label.set_text_color",
            signature: "label.set_text_color(id, text_color)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "label.set_style",
            signature: "label.set_style(id, style)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "label.set_yloc",
            signature: "label.set_yloc(id, yloc)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "label.set_xloc",
            signature: "label.set_xloc(id, xloc, x)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "label.set_size",
            signature: "label.set_size(id, size)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "label.set_textalign",
            signature: "label.set_textalign(id, textalign)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "label.set_tooltip",
            signature: "label.set_tooltip(id, tooltip)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "label.set_text_formatting",
            signature: "label.set_text_formatting(id, text_formatting)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "label.set_text_font_family",
            signature: "label.set_text_font_family(id, font_family)",
            return_type: Type::scalar(BaseType::Void),
        },
    ]
}
