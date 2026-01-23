use crate::builtins::BuiltinFunction;
use crate::types::{BaseType, Type};

pub(super) fn list() -> Vec<BuiltinFunction> {
    vec![
        // strategy.opentrades.* functions
        BuiltinFunction {
            name: "strategy.opentrades.entry_bar_index",
            signature: "strategy.opentrades.entry_bar_index(trade_num)",
            return_type: Type::scalar(BaseType::Int),
        },
        BuiltinFunction {
            name: "strategy.opentrades.entry_comment",
            signature: "strategy.opentrades.entry_comment(trade_num)",
            return_type: Type::scalar(BaseType::String),
        },
        BuiltinFunction {
            name: "strategy.opentrades.entry_id",
            signature: "strategy.opentrades.entry_id(trade_num)",
            return_type: Type::scalar(BaseType::String),
        },
        BuiltinFunction {
            name: "strategy.opentrades.entry_price",
            signature: "strategy.opentrades.entry_price(trade_num)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "strategy.opentrades.entry_time",
            signature: "strategy.opentrades.entry_time(trade_num)",
            return_type: Type::scalar(BaseType::Int),
        },
        BuiltinFunction {
            name: "strategy.opentrades.commission",
            signature: "strategy.opentrades.commission(trade_num)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "strategy.opentrades.max_drawdown",
            signature: "strategy.opentrades.max_drawdown(trade_num)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "strategy.opentrades.max_runup",
            signature: "strategy.opentrades.max_runup(trade_num)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "strategy.opentrades.profit",
            signature: "strategy.opentrades.profit(trade_num)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "strategy.opentrades.profit_percent",
            signature: "strategy.opentrades.profit_percent(trade_num)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "strategy.opentrades.size",
            signature: "strategy.opentrades.size(trade_num)",
            return_type: Type::scalar(BaseType::Float),
        },
    ]
}
