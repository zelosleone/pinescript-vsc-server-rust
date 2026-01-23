use crate::builtins::BuiltinFunction;
use crate::types::{BaseType, Type};

pub(super) fn list() -> Vec<BuiltinFunction> {
    vec![
        // Matrix helpers - Transformation & Calculation functions
        BuiltinFunction {
            name: "matrix.submatrix",
            signature: "matrix.submatrix(m, from_row, to_row, from_col, to_col)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "matrix.reshape",
            signature: "matrix.reshape(m, rows, columns)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "matrix.reverse",
            signature: "matrix.reverse(m)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "matrix.transpose",
            signature: "matrix.transpose(m)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "matrix.sort",
            signature: "matrix.sort(m, column, order)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "matrix.concat",
            signature: "matrix.concat(m1, m2, dimension)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "matrix.avg",
            signature: "matrix.avg(m)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "matrix.min",
            signature: "matrix.min(m)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "matrix.max",
            signature: "matrix.max(m)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "matrix.mode",
            signature: "matrix.mode(m)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "matrix.median",
            signature: "matrix.median(m)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "matrix.sum",
            signature: "matrix.sum(m1, m2)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "matrix.diff",
            signature: "matrix.diff(m1, m2)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "matrix.mult",
            signature: "matrix.mult(m1, m2)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "matrix.pow",
            signature: "matrix.pow(m, power)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "matrix.det",
            signature: "matrix.det(m)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "matrix.inv",
            signature: "matrix.inv(m)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "matrix.pinv",
            signature: "matrix.pinv(m)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "matrix.rank",
            signature: "matrix.rank(m)",
            return_type: Type::scalar(BaseType::Int),
        },
        BuiltinFunction {
            name: "matrix.trace",
            signature: "matrix.trace(m)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "matrix.eigenvalues",
            signature: "matrix.eigenvalues(m)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "matrix.eigenvectors",
            signature: "matrix.eigenvectors(m)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "matrix.kron",
            signature: "matrix.kron(m1, m2)",
            return_type: Type::unknown(),
        },
    ]
}
