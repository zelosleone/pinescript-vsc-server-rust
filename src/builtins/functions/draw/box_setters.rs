use crate::builtins::BuiltinFunction;
use crate::types::{BaseType, Type};

pub(super) fn list() -> Vec<BuiltinFunction> {
    vec![
        // Box setters
        BuiltinFunction {
            name: "box.set_top",
            signature: "box.set_top(id, top)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "box.set_bottom",
            signature: "box.set_bottom(id, bottom)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "box.set_left",
            signature: "box.set_left(id, left)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "box.set_right",
            signature: "box.set_right(id, right)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "box.set_lefttop",
            signature: "box.set_lefttop(id, left, top)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "box.set_rightbottom",
            signature: "box.set_rightbottom(id, right, bottom)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "box.set_top_left_point",
            signature: "box.set_top_left_point(id, point)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "box.set_bottom_right_point",
            signature: "box.set_bottom_right_point(id, point)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "box.set_border_color",
            signature: "box.set_border_color(id, color)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "box.set_border_width",
            signature: "box.set_border_width(id, width)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "box.set_border_style",
            signature: "box.set_border_style(id, style)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "box.set_extend",
            signature: "box.set_extend(id, extend)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "box.set_bgcolor",
            signature: "box.set_bgcolor(id, color)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "box.set_text",
            signature: "box.set_text(id, text)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "box.set_text_color",
            signature: "box.set_text_color(id, color)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "box.set_text_size",
            signature: "box.set_text_size(id, text_size)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "box.set_text_halign",
            signature: "box.set_text_halign(id, text_halign)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "box.set_text_valign",
            signature: "box.set_text_valign(id, text_valign)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "box.set_text_wrap",
            signature: "box.set_text_wrap(id, text_wrap)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "box.set_xloc",
            signature: "box.set_xloc(id, xloc, x1, x2)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "box.set_text_formatting",
            signature: "box.set_text_formatting(id, text_formatting)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "box.set_text_font_family",
            signature: "box.set_text_font_family(id, font_family)",
            return_type: Type::scalar(BaseType::Void),
        },
    ]
}
