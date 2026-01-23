use crate::builtins::BuiltinFunction;
use crate::types::{BaseType, Type};

pub(super) fn list() -> Vec<BuiltinFunction> {
    vec![
        // String helpers
        BuiltinFunction {
            name: "str.tostring",
            signature: "str.tostring(x)",
            return_type: Type::scalar(BaseType::String),
        },
        BuiltinFunction {
            name: "str.format",
            signature: "str.format(template, args...)",
            return_type: Type::scalar(BaseType::String),
        },
        BuiltinFunction {
            name: "str.format_time",
            signature: "str.format_time(time, format, timezone?)",
            return_type: Type::scalar(BaseType::String),
        },
        BuiltinFunction {
            name: "str.contains",
            signature: "str.contains(s, substr)",
            return_type: Type::scalar(BaseType::Bool),
        },
        BuiltinFunction {
            name: "str.replace",
            signature: "str.replace(s, from, to)",
            return_type: Type::scalar(BaseType::String),
        },
        BuiltinFunction {
            name: "str.split",
            signature: "str.split(s, sep)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "str.join",
            signature: "str.join(delim, arr)",
            return_type: Type::scalar(BaseType::String),
        },
        BuiltinFunction {
            name: "str.tonumber",
            signature: "str.tonumber(s)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "str.replace_all",
            signature: "str.replace_all(source, target, replacement)",
            return_type: Type::scalar(BaseType::String),
        },
        BuiltinFunction {
            name: "str.upper",
            signature: "str.upper(s)",
            return_type: Type::scalar(BaseType::String),
        },
        BuiltinFunction {
            name: "str.lower",
            signature: "str.lower(s)",
            return_type: Type::scalar(BaseType::String),
        },
        BuiltinFunction {
            name: "str.trim",
            signature: "str.trim(s)",
            return_type: Type::scalar(BaseType::String),
        },
        BuiltinFunction {
            name: "str.repeat",
            signature: "str.repeat(s, repeat, separator?)",
            return_type: Type::scalar(BaseType::String),
        },
        BuiltinFunction {
            name: "str.length",
            signature: "str.length(s)",
            return_type: Type::scalar(BaseType::Int),
        },
        BuiltinFunction {
            name: "str.pos",
            signature: "str.pos(source, str)",
            return_type: Type::scalar(BaseType::Int),
        },
        BuiltinFunction {
            name: "str.indexof",
            signature: "str.indexof(source, str)",
            return_type: Type::scalar(BaseType::Int),
        },
        BuiltinFunction {
            name: "str.substring",
            signature: "str.substring(source, begin_pos, end_pos?)",
            return_type: Type::scalar(BaseType::String),
        },
        BuiltinFunction {
            name: "str.startswith",
            signature: "str.startswith(source, str)",
            return_type: Type::scalar(BaseType::Bool),
        },
        BuiltinFunction {
            name: "str.endswith",
            signature: "str.endswith(source, str)",
            return_type: Type::scalar(BaseType::Bool),
        },
        BuiltinFunction {
            name: "str.match",
            signature: "str.match(source, regex)",
            return_type: Type::scalar(BaseType::String),
        },
        BuiltinFunction {
            name: "str.match_all",
            signature: "str.match_all(source, regex)",
            return_type: Type::unknown(),
        },
    ]
}
