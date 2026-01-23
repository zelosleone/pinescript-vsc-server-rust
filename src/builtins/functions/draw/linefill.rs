use crate::builtins::BuiltinFunction;
use crate::types::{BaseType, Type};

pub(super) fn list() -> Vec<BuiltinFunction> {
    vec![
        // Linefill functions
        BuiltinFunction {
            name: "linefill.new",
            signature: "linefill.new(line1, line2, color)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "linefill.delete",
            signature: "linefill.delete(id)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "linefill.set_color",
            signature: "linefill.set_color(id, color)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "linefill.get_line1",
            signature: "linefill.get_line1(id)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "linefill.get_line2",
            signature: "linefill.get_line2(id)",
            return_type: Type::unknown(),
        },
    ]
}
