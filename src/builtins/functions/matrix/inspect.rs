use crate::builtins::BuiltinFunction;
use crate::types::{BaseType, Type};

pub(super) fn list() -> Vec<BuiltinFunction> {
    vec![
        // Matrix helpers - Inspection functions
        BuiltinFunction {
            name: "matrix.rows",
            signature: "matrix.rows(m)",
            return_type: Type::scalar(BaseType::Int),
        },
        BuiltinFunction {
            name: "matrix.columns",
            signature: "matrix.columns(m)",
            return_type: Type::scalar(BaseType::Int),
        },
        BuiltinFunction {
            name: "matrix.elements_count",
            signature: "matrix.elements_count(m)",
            return_type: Type::scalar(BaseType::Int),
        },
        BuiltinFunction {
            name: "matrix.is_square",
            signature: "matrix.is_square(m)",
            return_type: Type::scalar(BaseType::Bool),
        },
        BuiltinFunction {
            name: "matrix.is_identity",
            signature: "matrix.is_identity(m)",
            return_type: Type::scalar(BaseType::Bool),
        },
        BuiltinFunction {
            name: "matrix.is_diagonal",
            signature: "matrix.is_diagonal(m)",
            return_type: Type::scalar(BaseType::Bool),
        },
        BuiltinFunction {
            name: "matrix.is_antidiagonal",
            signature: "matrix.is_antidiagonal(m)",
            return_type: Type::scalar(BaseType::Bool),
        },
        BuiltinFunction {
            name: "matrix.is_symmetric",
            signature: "matrix.is_symmetric(m)",
            return_type: Type::scalar(BaseType::Bool),
        },
        BuiltinFunction {
            name: "matrix.is_antisymmetric",
            signature: "matrix.is_antisymmetric(m)",
            return_type: Type::scalar(BaseType::Bool),
        },
        BuiltinFunction {
            name: "matrix.is_triangular",
            signature: "matrix.is_triangular(m)",
            return_type: Type::scalar(BaseType::Bool),
        },
        BuiltinFunction {
            name: "matrix.is_stochastic",
            signature: "matrix.is_stochastic(m)",
            return_type: Type::scalar(BaseType::Bool),
        },
        BuiltinFunction {
            name: "matrix.is_binary",
            signature: "matrix.is_binary(m)",
            return_type: Type::scalar(BaseType::Bool),
        },
        BuiltinFunction {
            name: "matrix.is_zero",
            signature: "matrix.is_zero(m)",
            return_type: Type::scalar(BaseType::Bool),
        },
    ]
}
