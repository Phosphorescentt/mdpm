mod cli;
mod mdpm;
use clap::Parser;
use cli::Commands;

#[derive(Parser, Debug)]
#[command(name = "mdpm")]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

struct Project {
    title: String,
}

struct Task {
    title: String,
    project: Project,
}

fn main() {
    let args = Args::parse();
    return mdpm::handle_command(args.command);
}
