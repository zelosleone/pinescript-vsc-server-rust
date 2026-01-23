use crate::builtins::BuiltinFunction;

mod box_getters;
mod box_setters;
mod core;
mod label_getters;
mod label_setters;
mod line_getters;
mod line_setters;
mod linefill;
mod polyline;
mod table;

pub(super) fn list() -> Vec<BuiltinFunction> {
    let mut all = Vec::new();
    all.extend(core::list());
    all.extend(label_setters::list());
    all.extend(label_getters::list());
    all.extend(line_setters::list());
    all.extend(line_getters::list());
    all.extend(box_setters::list());
    all.extend(box_getters::list());
    all.extend(table::list());
    all.extend(polyline::list());
    all.extend(linefill::list());
    all
}
