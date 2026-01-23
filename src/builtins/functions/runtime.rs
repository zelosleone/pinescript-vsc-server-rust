use crate::builtins::BuiltinFunction;
use crate::types::{BaseType, Type};

pub(super) fn list() -> Vec<BuiltinFunction> {
    vec![
        // Runtime functions
        BuiltinFunction {
            name: "runtime.error",
            signature: "runtime.error(message)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "log.info",
            signature: "log.info(message)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "log.warning",
            signature: "log.warning(message)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "log.error",
            signature: "log.error(message)",
            return_type: Type::scalar(BaseType::Void),
        },
    ]
}
