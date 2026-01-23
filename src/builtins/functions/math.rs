use crate::builtins::BuiltinFunction;
use crate::types::{BaseType, Type};

pub(super) fn list() -> Vec<BuiltinFunction> {
    vec![
        // Math helpers
        BuiltinFunction {
            name: "math.pow",
            signature: "math.pow(x, y)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "math.sqrt",
            signature: "math.sqrt(x)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "math.round",
            signature: "math.round(x)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "math.floor",
            signature: "math.floor(x)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "math.ceil",
            signature: "math.ceil(x)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "math.log",
            signature: "math.log(x)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "math.exp",
            signature: "math.exp(x)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "math.abs",
            signature: "math.abs(x)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "math.max",
            signature: "math.max(x1, x2, ...)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "math.min",
            signature: "math.min(x1, x2, ...)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "math.sign",
            signature: "math.sign(x)",
            return_type: Type::scalar(BaseType::Int),
        },
        BuiltinFunction {
            name: "math.sin",
            signature: "math.sin(x)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "math.cos",
            signature: "math.cos(x)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "math.tan",
            signature: "math.tan(x)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "math.asin",
            signature: "math.asin(x)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "math.acos",
            signature: "math.acos(x)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "math.atan",
            signature: "math.atan(x)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "math.log10",
            signature: "math.log10(x)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "math.random",
            signature: "math.random(min?, max?, seed?)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "math.round_to_mintick",
            signature: "math.round_to_mintick(x)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "math.todegrees",
            signature: "math.todegrees(x)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "math.toradians",
            signature: "math.toradians(x)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "math.avg",
            signature: "math.avg(x1, x2, ...)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "math.sum",
            signature: "math.sum(x, length)",
            return_type: Type::series(BaseType::Float),
        },
    ]
}
