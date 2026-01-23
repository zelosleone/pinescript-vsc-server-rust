use crate::builtins::BuiltinFunction;
use crate::types::Type;

pub(super) fn list() -> Vec<BuiltinFunction> {
    vec![
        BuiltinFunction {
            name: "chart.point.from_index",
            signature: "chart.point.from_index(index, price)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "chart.point.from_time",
            signature: "chart.point.from_time(time, price)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "chart.point.now",
            signature: "chart.point.now(price)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "chart.point.new",
            signature: "chart.point.new(x, price)",
            return_type: Type::unknown(),
        },
    ]
}
