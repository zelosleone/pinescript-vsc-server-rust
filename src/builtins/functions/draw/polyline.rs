use crate::builtins::BuiltinFunction;
use crate::types::{BaseType, Type};

pub(super) fn list() -> Vec<BuiltinFunction> {
    vec![
        // Polyline functions
        BuiltinFunction {
            name: "polyline.new",
            signature: "polyline.new(points, curved?, closed?, xloc?, line_color?, fill_color?, line_style?, line_width?, force_overlay?)",
            return_type: Type::unknown(),
        },
        BuiltinFunction {
            name: "polyline.delete",
            signature: "polyline.delete(id)",
            return_type: Type::scalar(BaseType::Void),
        },
    ]
}
