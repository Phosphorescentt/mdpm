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
    // make sure the args are valid first!
    let args = Args::parse();

    // load config
    let config = mdpm::load_config();

    // load store_list into memory
    let mut stores = mdpm::load_stores(&config);

    return mdpm::handle_command(args.command, &mut stores, config).unwrap();
}
