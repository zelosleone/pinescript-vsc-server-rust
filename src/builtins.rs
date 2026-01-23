use std::collections::HashMap;

use crate::types::Type;

mod functions;
mod values;

#[derive(Clone, Debug)]
pub struct BuiltinFunction {
    pub name: &'static str,
    pub signature: &'static str,
    pub return_type: Type,
}

#[derive(Clone, Debug)]
pub struct Builtins {
    functions: HashMap<String, BuiltinFunction>,
    values: HashMap<String, Type>,
}

impl Builtins {
    pub fn new() -> Self {
        let mut functions = HashMap::new();
        for func in functions::builtin_functions() {
            functions.insert(func.name.to_string(), func);
        }

        let mut values = HashMap::new();
        for (name, ty) in values::builtin_values() {
            values.insert(name.to_string(), ty);
        }

        Self { functions, values }
    }

    pub fn function(&self, name: &str) -> Option<&BuiltinFunction> {
        self.functions.get(name)
    }

    pub fn is_function(&self, name: &str) -> bool {
        self.functions.contains_key(name)
    }

    pub fn value_type(&self, name: &str) -> Option<&Type> {
        self.values.get(name)
    }

    pub fn values(&self) -> impl Iterator<Item = (&String, &Type)> {
        self.values.iter()
    }

    pub fn functions(&self) -> impl Iterator<Item = &BuiltinFunction> {
        self.functions.values()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtins_include_core_v6_items() {
        let b = Builtins::new();
        assert!(
            b.value_type("syminfo.isin").is_some(),
            "syminfo.isin missing"
        );
        assert!(
            b.value_type("plot.linestyle_solid").is_some(),
            "plot.linestyle_solid missing"
        );
        assert!(
            b.value_type("text.format_bold").is_some(),
            "text.format_bold missing"
        );

        assert!(b.function("library").is_some(), "library missing");
        assert!(b.function("input.enum").is_some(), "input.enum missing");
        assert!(
            b.function("str.match_all").is_some(),
            "str.match_all missing"
        );
        assert!(b.function("str.indexof").is_some(), "str.indexof missing");
        assert!(
            b.function("chart.point.from_index").is_some(),
            "chart.point.from_index missing"
        );
        assert!(
            b.function("time")
                .map(|f| f.signature.contains("timeframe_bars_back"))
                .unwrap_or(false),
            "time missing timeframe_bars_back param"
        );
        assert!(
            b.function("plot")
                .map(|f| f.signature.contains("force_overlay"))
                .unwrap_or(false),
            "plot missing force_overlay param"
        );
        assert!(
            b.function("label.new")
                .map(|f| f.signature.contains("text_formatting"))
                .unwrap_or(false),
            "label.new missing text_formatting param"
        );
        assert!(
            b.function("request.security")
                .map(|f| f.signature.contains("gaps"))
                .unwrap_or(false),
            "request.security missing gaps param"
        );
        assert!(
            b.function("request.security")
                .map(|f| f.signature.contains("lookahead"))
                .unwrap_or(false),
            "request.security missing lookahead param"
        );
    }
}
