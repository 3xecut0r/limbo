use crate::kernel::field::Field;


pub trait Model {
    const NAME: &'static str;

    fn schema() -> Vec<Field>;
}