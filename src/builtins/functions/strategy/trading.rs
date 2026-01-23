use crate::builtins::BuiltinFunction;
use crate::types::{BaseType, Type};

pub(super) fn list() -> Vec<BuiltinFunction> {
    vec![
        // Strategy trading functions
        BuiltinFunction {
            name: "strategy.entry",
            signature: "strategy.entry(id, direction, qty?, limit?, stop?, oca_name?, oca_type?, comment?, alert_message?, disable_alert?)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "strategy.exit",
            signature: "strategy.exit(id, from_entry?, qty?, qty_percent?, profit?, limit?, loss?, stop?, trail_price?, trail_points?, trail_offset?, oca_name?, comment?, comment_profit?, comment_loss?, comment_trailing?, alert_message?, alert_profit?, alert_loss?, alert_trailing?, disable_alert?)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "strategy.close",
            signature: "strategy.close(id, comment?, alert_message?, immediately?, disable_alert?)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "strategy.close_all",
            signature: "strategy.close_all(comment?, alert_message?, immediately?, disable_alert?)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "strategy.cancel",
            signature: "strategy.cancel(id)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "strategy.cancel_all",
            signature: "strategy.cancel_all()",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "strategy.order",
            signature: "strategy.order(id, direction, qty?, limit?, stop?, oca_name?, oca_type?, comment?, alert_message?, disable_alert?)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "strategy.risk.allow_entry_in",
            signature: "strategy.risk.allow_entry_in(value)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "strategy.risk.max_cons_loss_days",
            signature: "strategy.risk.max_cons_loss_days(count)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "strategy.risk.max_drawdown",
            signature: "strategy.risk.max_drawdown(value, type)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "strategy.risk.max_intraday_filled_orders",
            signature: "strategy.risk.max_intraday_filled_orders(count)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "strategy.risk.max_intraday_loss",
            signature: "strategy.risk.max_intraday_loss(value, type)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "strategy.risk.max_position_size",
            signature: "strategy.risk.max_position_size(contracts)",
            return_type: Type::scalar(BaseType::Void),
        },
    ]
}
