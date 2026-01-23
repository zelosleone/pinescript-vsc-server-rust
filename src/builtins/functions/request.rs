use crate::builtins::BuiltinFunction;
use crate::types::{BaseType, Type};

pub(super) fn list() -> Vec<BuiltinFunction> {
    vec![
        // Additional request.* functions
        BuiltinFunction {
            name: "request.dividends",
            signature: "request.dividends(ticker?, field?, gaps?, lookahead?, ignore_invalid_symbol?, currency?)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "request.earnings",
            signature: "request.earnings(ticker?, field?, gaps?, lookahead?, ignore_invalid_symbol?, currency?)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "request.financial",
            signature: "request.financial(symbol, financial_id, period, gaps?, ignore_invalid_symbol?, currency?)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "request.quandl",
            signature: "request.quandl(ticker, gaps?, index?, ignore_invalid_symbol?)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "request.splits",
            signature: "request.splits(ticker?, field?, gaps?, lookahead?, ignore_invalid_symbol?)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "request.currency_rate",
            signature: "request.currency_rate(from, to, ignore_invalid_symbol?)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "request.seed",
            signature: "request.seed(source, symbol, expression, ignore_invalid_symbol?)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "request.security_lower_tf",
            signature: "request.security_lower_tf(symbol, timeframe, expression, ignore_invalid_symbol?, currency?, ignore_invalid_timeframe?)",
            return_type: Type::unknown(),
        },
    ]
}
