use crate::builtins::BuiltinFunction;
use crate::types::{BaseType, Type};

pub(super) fn list() -> Vec<BuiltinFunction> {
    vec![
        BuiltinFunction {
            name: "library",
            signature: "library(title, overlay?)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "indicator",
            signature: "indicator(title, shorttitle?, overlay?)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "strategy",
            signature: "strategy(title, shorttitle?, overlay?)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "time",
            signature: "time(timeframe?, session?, bars_back?, timeframe_bars_back?)",
            return_type: Type::series(BaseType::Int),
        },
        BuiltinFunction {
            name: "time_close",
            signature: "time_close(timeframe?, session?, bars_back?, timeframe_bars_back?)",
            return_type: Type::series(BaseType::Int),
        },
        BuiltinFunction {
            name: "plot",
            signature: "plot(series, title?, color?, linewidth?, style?, trackprice?, histbase?, offset?, join?, editable?, show_last?, display?, format?, precision?, force_overlay?, linestyle?)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "plotchar",
            signature: "plotchar(series, title?, char?, location?, color?, size?, offset?, editable?, show_last?, display?, format?, precision?)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "plotshape",
            signature: "plotshape(series, title?, style?, location?, color?, text?, textcolor?, size?, offset?, editable?, show_last?, display?, format?, precision?)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "plotcandle",
            signature: "plotcandle(open, high, low, close, title?, color?, wickcolor?, editable?, show_last?, bordercolor?, display?)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "plotbar",
            signature: "plotbar(open, high, low, close, title?, color?, editable?, show_last?, bordercolor?, display?, force_overlay?)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "plotarrow",
            signature: "plotarrow(series, title?, colorup?, colordown?, offset?, minheight?, maxheight?, editable?, show_last?, display?)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "fill",
            signature: "fill(hline1, hline2, color?)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "hline",
            signature: "hline(price, title?, color?, linewidth?, linestyle?, display?)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "barcolor",
            signature: "barcolor(color?, offset?, editable?, show_last?, title?, display?)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "bgcolor",
            signature: "bgcolor(color?, offset?, editable?, show_last?, title?, display?, force_overlay?)",
            return_type: Type::scalar(BaseType::Void),
        },
        BuiltinFunction {
            name: "max_bars_back",
            signature: "max_bars_back(series, length)",
            return_type: Type::scalar(BaseType::Void),
        },
    ]
}
