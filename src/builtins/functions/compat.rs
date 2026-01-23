use crate::builtins::BuiltinFunction;
use crate::types::{BaseType, Type};

pub(super) fn list() -> Vec<BuiltinFunction> {
    vec![
        // Existing/compat
        BuiltinFunction {
            name: "nz",
            signature: "nz(x, replacement?)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "na",
            signature: "na(x)",
            return_type: Type::scalar(BaseType::Bool),
        },
        BuiltinFunction {
            name: "request.security",
            signature: "request.security(symbol, timeframe, expression, gaps?, lookahead?, ignore_invalid_symbol?, currency?)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "input.int",
            signature: "input.int(defval, title?, minval?, maxval?, step?, tooltip?, inline?, group?, confirm?, display?, active?)",
            return_type: Type::scalar(BaseType::Int),
        },
        BuiltinFunction {
            name: "input.float",
            signature: "input.float(defval, title?, minval?, maxval?, step?, tooltip?, inline?, group?, confirm?, display?, active?)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "input.bool",
            signature: "input.bool(defval, title?, tooltip?, inline?, group?, confirm?, display?, active?)",
            return_type: Type::scalar(BaseType::Bool),
        },
        BuiltinFunction {
            name: "input.string",
            signature: "input.string(defval, title?, tooltip?, group?, confirm?, display?, active?)",
            return_type: Type::scalar(BaseType::String),
        },
        BuiltinFunction {
            name: "input.source",
            signature: "input.source(defval, title?, tooltip?, inline?, group?, confirm?, display?, active?)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "input.color",
            signature: "input.color(defval, title?, tooltip?, inline?, group?, confirm?, display?, active?)",
            return_type: Type::scalar(BaseType::Color),
        },
        BuiltinFunction {
            name: "alert",
            signature: "alert(condition, message?)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "alertcondition",
            signature: "alertcondition(condition, title?)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "timestamp",
            signature: "timestamp(year, month, day, hour?, minute?)",
            return_type: Type::scalar(BaseType::Int),
        },
    ]
}
