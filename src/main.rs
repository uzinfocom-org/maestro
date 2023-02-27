mod args;
mod config;
mod http;

use args::{Cli, Commands};
use clap::Parser;
use std::path::Path;
use std::process::exit;

static TEMPLATE: &str = include_str!("../example.config.toml");

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Config { file } => {
            let location = Path::new(&file);

            let data = if location.exists() {
                config::Data::new(location.to_path_buf()).config
            } else {
                eprintln!("File does not exist");
                exit(1)
            };

            let mut instance = http::Instance::new();
            instance.login(data.login, data.password)
        }
        Commands::Credentials { login, password } => {
            if login.is_empty() || password.is_empty() {
                eprintln!("Login or password can't be empty");
                exit(1)
            }

            let mut instance = http::Instance::new();
            instance.login(login, password)
        }
        Commands::Create => {
            // Create example config file in current directory using template
            let location = Path::new("example.config.toml");
            
            match location.exists() {
                true => {
                    eprintln!("File already exists");
                    exit(1)
                }
                false => {
                    std::fs::write(location, TEMPLATE).unwrap();
                }
            }
        }
    }
}
