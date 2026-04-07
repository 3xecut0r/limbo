use crate::kernel::field::FieldKind;

pub mod fields {
    use super::FieldKind;

    pub fn int() -> FieldKind {
        FieldKind::Int
    }

    pub fn float() -> FieldKind {
        FieldKind::Float
    }

    pub fn string(max_length: usize) -> FieldKind {
        FieldKind::String { max_length }
    }

    pub fn text() -> FieldKind {
        FieldKind::Text
    }

    pub fn boolean() -> FieldKind {
        FieldKind::Bool
    }

    pub fn many2one(target: &'static str) -> FieldKind {
        FieldKind::Many2One { target }
    }

    pub fn one2many(target: &'static str) -> FieldKind {
        FieldKind::One2Many { target }
    }

}