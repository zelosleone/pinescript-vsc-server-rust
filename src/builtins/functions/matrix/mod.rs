use crate::builtins::BuiltinFunction;

mod access;
mod create;
mod inspect;
mod row_column;
mod transform;

pub(super) fn list() -> Vec<BuiltinFunction> {
    let mut all = Vec::new();
    all.extend(create::list());
    all.extend(access::list());
    all.extend(row_column::list());
    all.extend(inspect::list());
    all.extend(transform::list());
    all
}
