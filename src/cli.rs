use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Clone, Debug, Subcommand)]
pub enum Commands {
    /// Initialises a new mdpm store.
    Init { store_name: String },
    /// Creates a new file.
    New { title: String },
}
