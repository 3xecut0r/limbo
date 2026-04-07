use crate::kernel::field::Field;


pub trait Model {
    const NAME: &'static str;
    fn schema() -> Vec<Field>;
}

pub struct ModelEntry {
    pub model_name: &'static str,
    pub schema_fn: fn() -> Vec<Field>,
}


inventory::collect!(ModelEntry);
