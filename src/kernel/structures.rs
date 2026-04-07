use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};


pub const CONFIG_FILE: &str = "limbo.toml";

#[derive(Parser)]
#[command(name = "limbo")]
#[command(version = "1.0.0.1")]
#[command(about = "CLI tool example")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init,
    Create {
        #[command(subcommand)]
        target: CreateCommands,
    },
    Start,
    // Stop,
    // Restart
}

#[derive(Subcommand)]
pub enum CreateCommands {
    Conf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub src_dir: String,
    pub db_name: String,
    pub db_host: String,
    pub db_port: u16,
    pub db_user: String,
    pub db_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DbConfig {
    pub db_name: String,
    pub db_host: String,
    pub db_port: u16,
    pub db_user: String,
    pub db_password: String,
}