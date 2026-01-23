use crate::builtins::BuiltinFunction;

mod closed_trades;
mod open_trades;
mod trading;

pub(super) fn list() -> Vec<BuiltinFunction> {
    let mut all = Vec::new();
    all.extend(trading::list());
    all.extend(closed_trades::list());
    all.extend(open_trades::list());
    all
}
