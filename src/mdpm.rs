use std::collections::HashMap;
use std::io::Write;
use std::{
    fs::{self, create_dir_all},
    io,
    path::{Path, PathBuf},
};

use crate::cli::Commands;

const MDPM_SHARE: &str = "~/.local/share/mdpm/";
const STORE_LINES: &str = "store.lines";

pub struct Config {
    mdpm_share: PathBuf,
}

pub struct Store {
    path: PathBuf,
    // If this is `None` then we haven't initial
    tickets: Option<Vec<Ticket>>,
}

impl Store {
    fn load_tickets(&mut self) -> io::Result<()> {
        todo!()
    }
}

impl From<PathBuf> for Store {
    fn from(value: PathBuf) -> Self {
        Store {
            path: value,
            tickets: None,
        }
    }
}

pub struct Ticket;

pub fn load_config() -> Config {
    let mdpm_share_dir = PathBuf::from(shellexpand::tilde(MDPM_SHARE).parse::<String>().unwrap());
    if !fs::exists(&mdpm_share_dir).unwrap() {
        fs::create_dir_all(&mdpm_share_dir).unwrap();
    }

    Config {
        mdpm_share: mdpm_share_dir,
    }
}

pub fn load_stores(config: &Config) -> HashMap<PathBuf, Store> {
    let store_lines_path = config.mdpm_share.clone().join(STORE_LINES);
    let store_map: HashMap<PathBuf, Store>;
    if fs::exists(&store_lines_path).unwrap() {
        store_map = HashMap::from_iter(
            fs::read_to_string(store_lines_path)
                .unwrap()
                .split("\n")
                .map(|s| {
                    let store_path_buf = PathBuf::from(s.replace("\n", ""));
                    (store_path_buf.clone(), Store::from(store_path_buf))
                }),
        );
    } else {
        fs::create_dir_all(&config.mdpm_share).unwrap();
        fs::File::create(&store_lines_path).unwrap();
        store_map = HashMap::new();
    }

    store_map
}

pub fn append_to_store_list(
    path: PathBuf,
    stores: HashMap<PathBuf, Store>,
    config: Config,
) -> io::Result<()> {
    if let Some(_store) = stores.get(&path) {
        // If we already ahve something, return.
        return Ok(());
    } else {
        let store_lines_path = config.mdpm_share.join(STORE_LINES);
        let mut store_lines_file = fs::OpenOptions::new()
            .append(true)
            .open(&store_lines_path)
            .unwrap();

        let cwd = std::env::current_dir().unwrap();

        store_lines_file.write_all(cwd.join(path).to_str().unwrap().as_bytes());
        store_lines_file.write_all("\n".as_bytes());
    }

    Ok(())
}

pub fn handle_command(
    command: Commands,
    stores: HashMap<PathBuf, Store>,
    config: Config,
) -> io::Result<()> {
    match command {
        Commands::Init { store_dir_name } => {
            let dir_to_create: PathBuf;
            if let Some(store_dir) = store_dir_name {
                dir_to_create = store_dir;
            } else {
                dir_to_create = PathBuf::from(".mdpm");
            }

            fs::create_dir(&dir_to_create).unwrap();
            append_to_store_list(dir_to_create, stores, config);
        }
        _ => todo!(),
    }

    Ok(())
}
