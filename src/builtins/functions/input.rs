use crate::builtins::BuiltinFunction;
use crate::types::{BaseType, Type};

pub(super) fn list() -> Vec<BuiltinFunction> {
    vec![
        // Additional input.* functions
        BuiltinFunction {
            name: "input.session",
            signature: "input.session(defval, title?, tooltip?, inline?, group?, confirm?, display?, active?)",
            return_type: Type::scalar(BaseType::String),
        },
        BuiltinFunction {
            name: "input.symbol",
            signature: "input.symbol(defval, title?, tooltip?, inline?, group?, confirm?, display?, active?)",
            return_type: Type::scalar(BaseType::String),
        },
        BuiltinFunction {
            name: "input.timeframe",
            signature: "input.timeframe(defval, title?, tooltip?, inline?, group?, confirm?, options?, display?, active?)",
            return_type: Type::scalar(BaseType::String),
        },
        BuiltinFunction {
            name: "input.time",
            signature: "input.time(defval, title?, tooltip?, inline?, group?, confirm?, display?, active?)",
            return_type: Type::scalar(BaseType::Int),
        },
        BuiltinFunction {
            name: "input.price",
            signature: "input.price(defval, title?, tooltip?, inline?, group?, confirm?, active?)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "input.text_area",
            signature: "input.text_area(defval, title?, tooltip?, group?, confirm?, display?, active?)",
            return_type: Type::scalar(BaseType::String),
        },
        BuiltinFunction {
            name: "input.enum",
            signature: "input.enum(defval, title?, tooltip?, inline?, group?, confirm?, options?, display?, active?)",
            return_type: Type::scalar(BaseType::String),
        },
    ]
}
