use clap::{Parser, Subcommand};

/// Automated login manager for Kerio Control
#[derive(Debug, Parser)]
#[command(name = "maestro")]
#[command(about = "Automated login manager for Kerio Control", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Login via written config file
    #[command(arg_required_else_help = true)]
    Config {
        /// The remote to clone
        file: String,
    },

    /// Login via credential
    #[command(arg_required_else_help = true)]
    Credentials {
        /// URL of the kerio control panel
        url: String,
        
        /// The remote to target
        login: String,

        /// Given password for the account
        password: String,
    },
    
    /// Create example config file
    Create
}
