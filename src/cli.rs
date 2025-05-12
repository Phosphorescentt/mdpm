use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Clone, Debug, Subcommand)]
pub enum Commands {
    /// Initialises a new mdpm store.
    Init { store_dir_name: Option<PathBuf> },
    /// Creates a new task or project.
    #[command(subcommand)]
    New(NewCommands),
}

#[derive(Clone, Debug, Subcommand)]
#[command(flatten_help = true)]
pub enum NewCommands {
    /// Creates a new task
    Task { title: String, body: Option<String> },
}
