use crate::builtins::BuiltinFunction;
use crate::types::{BaseType, Type};

pub(super) fn list() -> Vec<BuiltinFunction> {
    vec![
        BuiltinFunction {
            name: "timeframe.change",
            signature: "timeframe.change(timeframe?)",
            return_type: Type::series(BaseType::Bool),
        },
        BuiltinFunction {
            name: "timeframe.in_seconds",
            signature: "timeframe.in_seconds(timeframe?)",
            return_type: Type::scalar(BaseType::Float),
        },
    ]
}
