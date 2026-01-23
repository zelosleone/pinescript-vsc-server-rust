use crate::builtins::BuiltinFunction;
use crate::types::{BaseType, Type};

pub(super) fn list() -> Vec<BuiltinFunction> {
    vec![
        // Array statistical functions
        BuiltinFunction {
            name: "array.avg",
            signature: "array.avg(arr)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "array.min",
            signature: "array.min(arr)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "array.max",
            signature: "array.max(arr)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "array.median",
            signature: "array.median(arr)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "array.mode",
            signature: "array.mode(arr)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "array.stdev",
            signature: "array.stdev(arr, biased?)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "array.sum",
            signature: "array.sum(arr)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "array.variance",
            signature: "array.variance(arr, biased?)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "array.range",
            signature: "array.range(arr)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "array.covariance",
            signature: "array.covariance(arr1, arr2)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "array.standardize",
            signature: "array.standardize(arr)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "array.abs",
            signature: "array.abs(arr)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "array.percentile_linear_interpolation",
            signature: "array.percentile_linear_interpolation(arr, percentage)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "array.percentile_nearest_rank",
            signature: "array.percentile_nearest_rank(arr, percentage)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "array.percentrank",
            signature: "array.percentrank(arr, value)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "array.first",
            signature: "array.first(arr)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "array.last",
            signature: "array.last(arr)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "array.every",
            signature: "array.every(arr, predicate)",
            return_type: Type::scalar(BaseType::Bool),
        },
        BuiltinFunction {
            name: "array.some",
            signature: "array.some(arr, predicate)",
            return_type: Type::scalar(BaseType::Bool),
        },
        BuiltinFunction {
            name: "array.join",
            signature: "array.join(arr, separator?)",
            return_type: Type::scalar(BaseType::String),
        },
    ]
}
