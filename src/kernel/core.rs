use std::{fs};

use super::structures;
use super::database::db::Database;
use crate::kernel::registry::regs::ModelRegistry;


pub fn read_config() {
    let text = match fs::read_to_string(structures::CONFIG_FILE) {
        Ok(text) => text,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    let config: structures::Config = match toml::from_str(&text) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    // db connection:
    let db_config = structures::DbConfig {
        db_name: config.db_name.clone(),
        db_host: config.db_host.clone(),
        db_port: config.db_port.clone(),
        db_user: config.db_user.clone(),
        db_password: config.db_password.clone(),
    };
    let pool = match Database::new(db_config) {
        Ok(pool) => pool,
        Err(err) => {
            eprintln!("Database init error: {}", err);
            return; // todo: probably better way to panic instead of return
        }
    };

    // registry init
    let registry = ModelRegistry::from_inventory();
    for model in registry.all() {
        println!("Loaded model: {}", model.name);
    }
    // todo: registry logic of sync with db
    // registry.sync_schema(&pool);
    // registry.validate();

    let _ = pool;
}


