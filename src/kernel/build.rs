use clap::{Parser};
use std::{fs};
use super::structures::{Cli, Commands, CreateCommands, Config, CONFIG_FILE};
use super::core;

pub fn parse_and_match_cli() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => {
            println!("Initializing project...")
        }

        Commands::Create { target } => match target {
            CreateCommands::Conf => {
                create_config();
            }
        }

        Commands::Start => {
            core::read_config();
        }
    }
}

fn create_config() {
    let config = Config {
        db_name: "".to_string(),
        db_host: "".to_string(),
        db_port: 5432,
        db_user: "".to_string(),
        db_password: "".to_string(),
    };

    let toml_string = toml::to_string_pretty(&config)
        .expect("Serializing config unsuccessful");

    fs::write(CONFIG_FILE, toml_string)
        .expect("Write config issue");

    println!("Config created: {}", CONFIG_FILE);
}

