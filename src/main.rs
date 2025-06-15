mod config;
mod tui;

use anyhow::Result;
use clap::Parser;
use crate::{config::Config};

#[derive(Parser)]
#[command(version, about = "RustReady TUI Core Template")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Launches the TUI
    Tui,
}

fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Tui => {
            let config_path = "templates/config.json";
            let config = Config::load(config_path).unwrap_or_default();
            tui::run_tui(config_path, config)?;
        }
    }

    Ok(())
}
