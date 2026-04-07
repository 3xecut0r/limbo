use std::{fs};

use super::structures;
use super::database::db::Database;


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

    let db_config = structures::DbConfig {
        db_name: config.db_name.clone(),
        db_host: config.db_host.clone(),
        db_port: config.db_port.clone(),
        db_user: config.db_user.clone(),
        db_password: config.db_password.clone(),
    };
    let pool = Database::new(db_config);
    pool.unwrap();
}


