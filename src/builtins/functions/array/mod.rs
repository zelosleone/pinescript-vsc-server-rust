use crate::builtins::BuiltinFunction;

mod constructors;
mod core;
mod mutation;
mod ordering;
mod search;
mod slice;
mod stats;

pub(super) fn list() -> Vec<BuiltinFunction> {
    let mut all = Vec::new();
    all.extend(core::list());
    all.extend(constructors::list());
    all.extend(mutation::list());
    all.extend(slice::list());
    all.extend(ordering::list());
    all.extend(search::list());
    all.extend(stats::list());
    all
}
