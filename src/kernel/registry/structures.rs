use crate::kernel::field::Field;

#[derive(Debug, Clone)]
pub struct ModelInfo {
    pub name: &'static str,
    pub fields: Vec<Field>,
}


