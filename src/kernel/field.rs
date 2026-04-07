#[derive(Debug, Clone)]
pub struct Field {
    pub name: &'static str,
    pub kind: FieldKind,
    pub nullable: bool,
    pub unique: bool,
}

impl Field {
    pub fn new(name: &'static str, kind: FieldKind) -> Self {
        Self {
            name,
            kind,
            nullable: false,
            unique: false,
        }
    }

    pub fn nullable(mut self, value: bool) -> Self {
        self.nullable = value;
        self
    }

    pub fn unique(mut self, value: bool) -> Self {
        self.unique = value;
        self
    }

}


#[derive(Debug, Clone)]
pub enum FieldKind {
    Int,
    Float,
    String { max_length: usize },
    Text,
    Bool,
    Many2One { target: &'static str },
    One2Many { target: &'static str },
}
