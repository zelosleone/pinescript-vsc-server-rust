use crate::builtins::BuiltinFunction;

mod array;
mod chart;
mod color;
mod compat;
mod core;
mod draw;
mod input;
mod map;
mod math;
mod matrix;
mod request;
mod runtime;
mod strategy;
mod string;
mod ta;
mod timeframe;

pub(super) fn builtin_functions() -> Vec<BuiltinFunction> {
    let mut all = Vec::new();
    all.extend(core::list());
    all.extend(chart::list());
    all.extend(ta::list());
    all.extend(timeframe::list());
    all.extend(math::list());
    all.extend(string::list());
    all.extend(array::list());
    all.extend(map::list());
    all.extend(matrix::list());
    all.extend(draw::list());
    all.extend(color::list());
    all.extend(runtime::list());
    all.extend(strategy::list());
    all.extend(input::list());
    all.extend(request::list());
    all.extend(compat::list());
    all
}
