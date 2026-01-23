use crate::builtins::BuiltinFunction;
use crate::types::Type;

pub(super) fn list() -> Vec<BuiltinFunction> {
    vec![
        // Array additional constructors
        BuiltinFunction {
            name: "array.new_string",
            signature: "array.new_string(size?, val?)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "array.new_color",
            signature: "array.new_color(size?, val?)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "array.new_line",
            signature: "array.new_line(size?)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "array.new_label",
            signature: "array.new_label(size?)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "array.new_box",
            signature: "array.new_box(size?)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "array.new_table",
            signature: "array.new_table(size?)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "array.new",
            signature: "array.new(size?, initial_value?)",
            return_type: Type::unknown(),
        },
    ]
}
