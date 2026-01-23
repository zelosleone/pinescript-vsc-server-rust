use crate::builtins::BuiltinFunction;
use crate::types::{BaseType, Type};

pub(super) fn list() -> Vec<BuiltinFunction> {
    vec![
        // Label/Line/Box/Table/Color
        BuiltinFunction {
            name: "label.new",
            signature: "label.new(point, text, xloc?, yloc?, color?, style?, size?, text_color?, text_formatting?, text_font_family?, force_overlay?) | label.new(x, y, text, xloc?, yloc?, color?, style?, size?, text_color?, text_formatting?, text_font_family?, force_overlay?)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "label.delete",
            signature: "label.delete(id)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "line.new",
            signature: "line.new(first_point, second_point, xloc?, extend?, color?, style?, width?, force_overlay?) | line.new(x1, y1, x2, y2, xloc?, extend?, color?, style?, width?, force_overlay?)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "box.new",
            signature: "box.new(top_left, bottom_right, border_color?, border_width?, border_style?, extend?, xloc?, bgcolor?, text?, text_size?, text_color?, text_halign?, text_valign?, text_wrap?, text_font_family?, force_overlay?, text_formatting?) | box.new(left, top, right, bottom, border_color?, border_width?, border_style?, extend?, xloc?, bgcolor?, text?, text_size?, text_color?, text_halign?, text_valign?, text_wrap?, text_font_family?, force_overlay?, text_formatting?)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "table.new",
            signature: "table.new(position, rows, cols, frame_color?, border_color?, bgcolor?, text_color?, text_size?, force_overlay?)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "color.new",
            signature: "color.new(color, transp?)",
            return_type: Type::scalar(BaseType::Color),
        },
    ]
}
