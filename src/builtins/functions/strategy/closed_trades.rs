use crate::builtins::BuiltinFunction;
use crate::types::{BaseType, Type};

pub(super) fn list() -> Vec<BuiltinFunction> {
    vec![
        // strategy.closedtrades.* functions
        BuiltinFunction {
            name: "strategy.closedtrades.entry_bar_index",
            signature: "strategy.closedtrades.entry_bar_index(trade_num)",
            return_type: Type::scalar(BaseType::Int),
        },
        BuiltinFunction {
            name: "strategy.closedtrades.entry_comment",
            signature: "strategy.closedtrades.entry_comment(trade_num)",
            return_type: Type::scalar(BaseType::String),
        },
        BuiltinFunction {
            name: "strategy.closedtrades.entry_id",
            signature: "strategy.closedtrades.entry_id(trade_num)",
            return_type: Type::scalar(BaseType::String),
        },
        BuiltinFunction {
            name: "strategy.closedtrades.entry_price",
            signature: "strategy.closedtrades.entry_price(trade_num)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "strategy.closedtrades.entry_time",
            signature: "strategy.closedtrades.entry_time(trade_num)",
            return_type: Type::scalar(BaseType::Int),
        },
        BuiltinFunction {
            name: "strategy.closedtrades.exit_bar_index",
            signature: "strategy.closedtrades.exit_bar_index(trade_num)",
            return_type: Type::scalar(BaseType::Int),
        },
        BuiltinFunction {
            name: "strategy.closedtrades.exit_comment",
            signature: "strategy.closedtrades.exit_comment(trade_num)",
            return_type: Type::scalar(BaseType::String),
        },
        BuiltinFunction {
            name: "strategy.closedtrades.exit_id",
            signature: "strategy.closedtrades.exit_id(trade_num)",
            return_type: Type::scalar(BaseType::String),
        },
        BuiltinFunction {
            name: "strategy.closedtrades.exit_price",
            signature: "strategy.closedtrades.exit_price(trade_num)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "strategy.closedtrades.exit_time",
            signature: "strategy.closedtrades.exit_time(trade_num)",
            return_type: Type::scalar(BaseType::Int),
        },
        BuiltinFunction {
            name: "strategy.closedtrades.commission",
            signature: "strategy.closedtrades.commission(trade_num)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "strategy.closedtrades.max_drawdown",
            signature: "strategy.closedtrades.max_drawdown(trade_num)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "strategy.closedtrades.max_runup",
            signature: "strategy.closedtrades.max_runup(trade_num)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "strategy.closedtrades.profit",
            signature: "strategy.closedtrades.profit(trade_num)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "strategy.closedtrades.profit_percent",
            signature: "strategy.closedtrades.profit_percent(trade_num)",
            return_type: Type::scalar(BaseType::Float),
        },
        BuiltinFunction {
            name: "strategy.closedtrades.size",
            signature: "strategy.closedtrades.size(trade_num)",
            return_type: Type::scalar(BaseType::Float),
        },
    ]
}
