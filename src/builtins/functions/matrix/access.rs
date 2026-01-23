use crate::builtins::BuiltinFunction;
use crate::types::{BaseType, Type};

pub(super) fn list() -> Vec<BuiltinFunction> {
    vec![
        // Matrix helpers - Element Access functions
        BuiltinFunction {
            name: "matrix.get",
            signature: "matrix.get(m, row, column)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "matrix.set",
            signature: "matrix.set(m, row, column, value)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "matrix.fill",
            signature: "matrix.fill(m, value, from_row?, to_row?, from_col?, to_col?)",
            return_type: Type::scalar(BaseType::Void),
        },
    ]
}
