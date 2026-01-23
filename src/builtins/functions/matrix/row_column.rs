use crate::builtins::BuiltinFunction;
use crate::types::{BaseType, Type};

pub(super) fn list() -> Vec<BuiltinFunction> {
    vec![
        // Matrix helpers - Row/Column Operations functions
        BuiltinFunction {
            name: "matrix.row",
            signature: "matrix.row(m, index)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "matrix.col",
            signature: "matrix.col(m, index)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "matrix.add_row",
            signature: "matrix.add_row(m, index, array?)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "matrix.add_col",
            signature: "matrix.add_col(m, index, array?)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "matrix.remove_row",
            signature: "matrix.remove_row(m, index)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "matrix.remove_col",
            signature: "matrix.remove_col(m, index)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "matrix.swap_rows",
            signature: "matrix.swap_rows(m, row1, row2)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "matrix.swap_columns",
            signature: "matrix.swap_columns(m, col1, col2)",
            return_type: Type::scalar(BaseType::Void),
        },
    ]
}
