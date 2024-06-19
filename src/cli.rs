use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Clone, Debug, Subcommand)]
pub enum Commands {
    /// Initialises a new mdpm store.
    Init { store_name: String },
    /// Creates a new task or project.
    #[command(subcommand)]
    New(NewCommands),
}

#[derive(Clone, Debug, Subcommand)]
#[command(flatten_help = true)]
pub enum NewCommands {
    /// Creates a new project
    Project { title: String, slug: String },
    /// Creates a new task
    Task {
        title: String,
        project: Option<String>,
    },
}
