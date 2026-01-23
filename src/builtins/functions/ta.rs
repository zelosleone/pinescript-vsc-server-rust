use crate::builtins::BuiltinFunction;
use crate::types::{BaseType, Type};

pub(super) fn list() -> Vec<BuiltinFunction> {
    vec![
        // TA functions
        BuiltinFunction {
            name: "ta.sma",
            signature: "ta.sma(source, length)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "ta.ema",
            signature: "ta.ema(source, length)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "ta.wma",
            signature: "ta.wma(source, length)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "ta.rma",
            signature: "ta.rma(source, length)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "ta.hma",
            signature: "ta.hma(source, length)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "ta.vwma",
            signature: "ta.vwma(source, length)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "ta.rsi",
            signature: "ta.rsi(source, length)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "ta.atr",
            signature: "ta.atr(length)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "ta.stdev",
            signature: "ta.stdev(source, length)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "ta.tr",
            signature: "ta.tr(handle_na?)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "ta.macd",
            signature: "ta.macd(source, fastlen, slowlen, siglen)",
            return_type: Type::Tuple(vec![
                Type::series(BaseType::Float),
                Type::series(BaseType::Float),
                Type::series(BaseType::Float),
            ]),
        },
        BuiltinFunction {
            name: "ta.crossover",
            signature: "ta.crossover(source1, source2)",
            return_type: Type::scalar(BaseType::Bool),
        },
        BuiltinFunction {
            name: "ta.crossunder",
            signature: "ta.crossunder(source1, source2)",
            return_type: Type::scalar(BaseType::Bool),
        },
        BuiltinFunction {
            name: "ta.cross",
            signature: "ta.cross(source1, source2)",
            return_type: Type::scalar(BaseType::Bool),
        },
        BuiltinFunction {
            name: "ta.highest",
            signature: "ta.highest(source, length)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "ta.lowest",
            signature: "ta.lowest(source, length)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "ta.highestbars",
            signature: "ta.highestbars(source, length)",
            return_type: Type::series(BaseType::Int),
        },
        BuiltinFunction {
            name: "ta.lowestbars",
            signature: "ta.lowestbars(source, length)",
            return_type: Type::series(BaseType::Int),
        },
        BuiltinFunction {
            name: "ta.barssince",
            signature: "ta.barssince(condition)",
            return_type: Type::series(BaseType::Int),
        },
        BuiltinFunction {
            name: "ta.adx",
            signature: "ta.adx(length)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "ta.obv",
            signature: "ta.obv()",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "ta.vwap",
            signature: "ta.vwap()",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "ta.cci",
            signature: "ta.cci(source, length)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "ta.mfi",
            signature: "ta.mfi(length)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "ta.roc",
            signature: "ta.roc(source, length)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "ta.williamsr",
            signature: "ta.williamsr(length)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "ta.stoch",
            signature: "ta.stoch(k, d, klen, dlen)",
            return_type: Type::Tuple(vec![
                Type::series(BaseType::Float),
                Type::series(BaseType::Float),
            ]),
        },
        BuiltinFunction {
            name: "ta.alma",
            signature: "ta.alma(source, windowsize, offset, sigma)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "ta.bb",
            signature: "ta.bb(source, length, mult)",
            return_type: Type::Tuple(vec![
                Type::series(BaseType::Float),
                Type::series(BaseType::Float),
                Type::series(BaseType::Float),
            ]),
        },
        BuiltinFunction {
            name: "ta.bbw",
            signature: "ta.bbw(source, length, mult)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "ta.change",
            signature: "ta.change(source, length?)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "ta.cmo",
            signature: "ta.cmo(source, length)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "ta.cog",
            signature: "ta.cog(source, length)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "ta.correlation",
            signature: "ta.correlation(source1, source2, length)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "ta.cum",
            signature: "ta.cum(source)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "ta.dev",
            signature: "ta.dev(source, length)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "ta.dmi",
            signature: "ta.dmi(length, lensig)",
            return_type: Type::Tuple(vec![
                Type::series(BaseType::Float),
                Type::series(BaseType::Float),
                Type::series(BaseType::Float),
            ]),
        },
        BuiltinFunction {
            name: "ta.falling",
            signature: "ta.falling(source, length)",
            return_type: Type::series(BaseType::Bool),
        },
        BuiltinFunction {
            name: "ta.rising",
            signature: "ta.rising(source, length)",
            return_type: Type::series(BaseType::Bool),
        },
        BuiltinFunction {
            name: "ta.kc",
            signature: "ta.kc(source, length, mult)",
            return_type: Type::Tuple(vec![
                Type::series(BaseType::Float),
                Type::series(BaseType::Float),
                Type::series(BaseType::Float),
            ]),
        },
        BuiltinFunction {
            name: "ta.kcw",
            signature: "ta.kcw(source, length, mult)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "ta.linreg",
            signature: "ta.linreg(source, length, offset)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "ta.max",
            signature: "ta.max(source)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "ta.min",
            signature: "ta.min(source)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "ta.median",
            signature: "ta.median(source, length)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "ta.mode",
            signature: "ta.mode(source, length)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "ta.mom",
            signature: "ta.mom(source, length)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "ta.percentile_linear_interpolation",
            signature: "ta.percentile_linear_interpolation(source, length, percentile)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "ta.percentile_nearest_rank",
            signature: "ta.percentile_nearest_rank(source, length, percentile)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "ta.percentrank",
            signature: "ta.percentrank(source, length)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "ta.pivot_point_levels",
            signature: "ta.pivot_point_levels(type, anchor)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "ta.pivothigh",
            signature: "ta.pivothigh(source?, leftbars, rightbars)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "ta.pivotlow",
            signature: "ta.pivotlow(source?, leftbars, rightbars)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "ta.range",
            signature: "ta.range(source, length)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "ta.rci",
            signature: "ta.rci(source, length)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "ta.sar",
            signature: "ta.sar(start, inc, max)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "ta.supertrend",
            signature: "ta.supertrend(factor, atrPeriod)",
            return_type: Type::Tuple(vec![
                Type::series(BaseType::Float),
                Type::series(BaseType::Float),
            ]),
        },
        BuiltinFunction {
            name: "ta.swma",
            signature: "ta.swma(source)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "ta.tsi",
            signature: "ta.tsi(source, fast, slow)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "ta.valuewhen",
            signature: "ta.valuewhen(condition, source, occurrence)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "ta.variance",
            signature: "ta.variance(source, length, biased?)",
            return_type: Type::series(BaseType::Float),
        },
        BuiltinFunction {
            name: "ta.wpr",
            signature: "ta.wpr(length)",
            return_type: Type::series(BaseType::Float),
        },
    ]
}
