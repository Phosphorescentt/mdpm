use std::path::PathBuf;

use clap::Subcommand;

#[derive(Clone, Debug, Subcommand)]
pub enum Commands {
    /// Initialises a new mdpm store in the current directory.
    Init,
    /// Creates a new task.
    New {
        title: String,
        body: Option<String>,
        filename: Option<PathBuf>,
    },
}
