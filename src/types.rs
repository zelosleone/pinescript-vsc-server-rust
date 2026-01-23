#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BaseType {
    Int,
    Float,
    Bool,
    String,
    Color,
    Void,
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Type {
    Scalar(BaseType),
    Series(BaseType),
    Tuple(Vec<Type>),
    /// A user-defined type (UDT or enum)
    UserDefined(String),
    Unknown,
}

impl Type {
    pub fn scalar(base: BaseType) -> Self {
        Type::Scalar(base)
    }

    pub fn series(base: BaseType) -> Self {
        Type::Series(base)
    }

    pub fn unknown() -> Self {
        Type::Unknown
    }

    pub fn is_series(&self) -> bool {
        matches!(self, Type::Series(_))
    }

    pub fn base(&self) -> BaseType {
        match self {
            Type::Scalar(base) | Type::Series(base) => base.clone(),
            Type::Tuple(_) => BaseType::Unknown,
            Type::Unknown => BaseType::Unknown,
            Type::UserDefined(_) => BaseType::Unknown,
        }
    }

    pub fn display_name(&self) -> String {
        match self {
            Type::Scalar(base) => base.display_name().to_string(),
            Type::Series(base) => format!("series<{}>", base.display_name()),
            Type::Tuple(items) => {
                let inner = items
                    .iter()
                    .map(|item| item.display_name())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("tuple<{}>", inner)
            }
            Type::UserDefined(name) => name.clone(),
            Type::Unknown => "unknown".to_string(),
        }
    }
}

impl BaseType {
    pub fn display_name(&self) -> &'static str {
        match self {
            BaseType::Int => "int",
            BaseType::Float => "float",
            BaseType::Bool => "bool",
            BaseType::String => "string",
            BaseType::Color => "color",
            BaseType::Void => "void",
            BaseType::Unknown => "unknown",
        }
    }
}
