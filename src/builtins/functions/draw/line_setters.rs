use crate::builtins::BuiltinFunction;
use crate::types::{BaseType, Type};

pub(super) fn list() -> Vec<BuiltinFunction> {
    vec![
        // Line setters
        BuiltinFunction {
            name: "line.set_x1",
            signature: "line.set_x1(id, x)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "line.set_y1",
            signature: "line.set_y1(id, y)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "line.set_x2",
            signature: "line.set_x2(id, x)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "line.set_y2",
            signature: "line.set_y2(id, y)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "line.set_xy1",
            signature: "line.set_xy1(id, x, y)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "line.set_xy2",
            signature: "line.set_xy2(id, x, y)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "line.set_xloc",
            signature: "line.set_xloc(id, xloc, x1, x2)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "line.set_extend",
            signature: "line.set_extend(id, extend)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "line.set_color",
            signature: "line.set_color(id, color)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "line.set_style",
            signature: "line.set_style(id, style)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "line.set_width",
            signature: "line.set_width(id, width)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "line.set_first_point",
            signature: "line.set_first_point(id, first_point)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "line.set_second_point",
            signature: "line.set_second_point(id, second_point)",
            return_type: Type::scalar(BaseType::Void),
        },
    ]
}
