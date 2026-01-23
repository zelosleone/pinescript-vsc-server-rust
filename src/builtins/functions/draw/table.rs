use crate::builtins::BuiltinFunction;
use crate::types::{BaseType, Type};

pub(super) fn list() -> Vec<BuiltinFunction> {
    vec![
        // Table functions
        BuiltinFunction {
            name: "table.cell",
            signature: "table.cell(id, column, row, text?, width?, height?, text_color?, text_halign?, text_valign?, text_size?, bgcolor?, tooltip?, text_font_family?, text_format?)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "table.set_position",
            signature: "table.set_position(id, position)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "table.set_frame_color",
            signature: "table.set_frame_color(id, color)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "table.set_frame_width",
            signature: "table.set_frame_width(id, width)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "table.set_border_color",
            signature: "table.set_border_color(id, color)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "table.set_border_width",
            signature: "table.set_border_width(id, width)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "table.set_bgcolor",
            signature: "table.set_bgcolor(id, color)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "table.cell_set_text",
            signature: "table.cell_set_text(id, column, row, text)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "table.cell_set_bgcolor",
            signature: "table.cell_set_bgcolor(id, column, row, bgcolor)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "table.cell_set_text_color",
            signature: "table.cell_set_text_color(id, column, row, text_color)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "table.cell_set_text_halign",
            signature: "table.cell_set_text_halign(id, column, row, text_halign)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "table.cell_set_text_valign",
            signature: "table.cell_set_text_valign(id, column, row, text_valign)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "table.cell_set_text_size",
            signature: "table.cell_set_text_size(id, column, row, text_size)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "table.cell_set_text_formatting",
            signature: "table.cell_set_text_formatting(id, column, row, text_formatting)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "table.cell_set_text_font_family",
            signature: "table.cell_set_text_font_family(id, column, row, font_family)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "table.cell_set_tooltip",
            signature: "table.cell_set_tooltip(id, column, row, tooltip)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "table.cell_set_width",
            signature: "table.cell_set_width(id, column, row, width)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "table.cell_set_height",
            signature: "table.cell_set_height(id, column, row, height)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "table.clear",
            signature: "table.clear(id, start_column, start_row, end_column, end_row)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "table.delete",
            signature: "table.delete(id)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "table.merge_cells",
            signature: "table.merge_cells(id, start_column, start_row, end_column, end_row)",
            return_type: Type::scalar(BaseType::Void),
        },
    ]
}
