use std::collections::HashMap;
use crate::kernel::registry::structures::ModelInfo;
use crate::kernel::models::models::{Model, ModelEntry};

pub struct ModelRegistry {
    models: HashMap<&'static str, ModelInfo>,
}

impl ModelRegistry {
    pub fn new() -> Self {
        Self {
            models: HashMap::new(),
        }
    }

    pub fn register<M: Model>(&mut self) {
        let info = ModelInfo {
            name: M::NAME,
            fields: M::schema(),
        };

        self.models.insert(M::NAME, info);
    }

    pub fn register_info(&mut self, info: ModelInfo) {
        self.models.insert(info.name, info);
    }

    pub fn get(&self, name: &str) -> Option<&ModelInfo> {
        self.models.get(name)
    }

    pub fn all(&self) -> impl Iterator<Item = &ModelInfo> {
        self.models.values()
    }

    pub fn init(&mut self) {
        for entry in inventory::iter::<ModelEntry> {
            let info = ModelInfo {
                name: entry.model_name,
                fields: (entry.schema_fn)(),
            };

            // for now is ok but in future would be replaced with check_inheritance and validate
            // todo: inheritance
            if self.models.contains_key(info.name) {
                panic!("Model '{}' already registered", info.name);
            }

            self.models.insert(info.name, info);
        }
    }

    pub fn from_inventory() -> Self {
        let mut registry = Self::new();
        registry.init();
        registry
    }
}

#[macro_export]
macro_rules! declare_model {
    (
        $name:ident, $model_name:literal, {
            $(
                $field_name:literal => $field_expr:expr
            ),* $(,)?
        }
    ) => {
        pub struct $name;

        impl $crate::kernel::models::models::Model for $name {
            const NAME: &'static str = $model_name;

            fn schema() -> Vec<$crate::kernel::field::Field> {
                vec![
                    $(
                        $crate::kernel::field::Field::new($field_name, $field_expr)
                    ),*
                ]
            }
        }

        inventory::submit! {
            $crate::kernel::models::models::ModelEntry {
                model_name: $name::NAME,
                schema_fn: $name::schema,
            }
        }
    };
}

// Usage
// let mut registry = ModelRegistry::new();
// registry.register::<User>();
// registry.register::<Settings>();
// registry.register::<Accounting>();

// collect all models and register them


