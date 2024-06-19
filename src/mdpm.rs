use std::{fs::File, io::Write};

use crate::cli::Commands;

pub fn handle_command(command: Commands) -> () {
    match command {
        Commands::Init { store_name } => init_store(store_name),
        Commands::New { title } => new_task(title),
    }
}

fn init_store(store_name: String) -> () {
    let file = File::create_new(".pm.md");
    match file {
        Ok(mut file) => {
            file.write_all(store_name.as_bytes()).unwrap();
            println!("Initialised mdpm store at current directory.")
        }
        Err(e) => {
            println!("{}", e);
            panic!("Something went wrong intialising the repo");
        }
    }
}

fn new_task(title: String) -> () {
    println!("Title: {}", title);
}
