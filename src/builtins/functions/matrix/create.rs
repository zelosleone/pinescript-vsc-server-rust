use crate::builtins::BuiltinFunction;
use crate::types::Type;

pub(super) fn list() -> Vec<BuiltinFunction> {
    vec![
        // Matrix helpers - Creation functions
        BuiltinFunction {
            name: "matrix.new",
            signature: "matrix.new<type>(rows, columns, initial_value?)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "matrix.copy",
            signature: "matrix.copy(m)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "matrix.from_array",
            signature: "matrix.from_array(array)",
            return_type: Type::unknown(),
        },
    ]
}
