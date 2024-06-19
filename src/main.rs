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

fn main() {
    let args = Args::parse();
    return mdpm::handle_command(args.command);
}

// Categories are groupings of tasks by team or area of code.
// Categories affect the slugs that the tasks recieve.
// Projects are logical groupings of tasks
// No work is stored in a project, only a set of tasks and a set of projects.
// Tasks are work to be done. This is the most granular level of work.
