use std::collections::HashMap;
use std::io::Write;
use std::ops::DerefMut;
use std::{
    fs::{self, DirEntry},
    io,
    path::PathBuf,
};

use crate::cli::Commands;

const MDPM_SHARE: &str = "~/.local/share/mdpm/";
const STORE_LINES: &str = "store.lines";

pub struct Config {
    mdpm_share: PathBuf,
}

#[derive(Debug)]
pub struct Store {
    path: PathBuf,
    // If this is `None` then we haven't hydrated the Store yet.
    tickets: Option<Vec<Ticket>>,
}

impl Store {
    fn hydrate(&mut self) -> () {
        // Populate tasks
        let paths = std::fs::read_dir(&self.path).unwrap().filter_map(|d| {
            let pa = d.as_ref().unwrap().path();
            if let Some(extension) = pa.extension() {
                // If we have an extension and it's ".md" and it's not a directory
                if extension == "md" && !pa.is_dir() {
                    Some(pa)
                } else {
                    None
                }
            } else {
                None
            }
        });

        let mut tickets = Vec::new();
        for path in paths {
            let content = fs::read_to_string(path).unwrap();

            // NOTE: We could probably make this much easier using Serde or something but I cba
            // right now :)
            let mut iter = content.split("\n");

            iter.next();
            let title = iter.next().unwrap().split(":").collect::<Vec<&str>>()[1];
            iter.next();
            let body = iter.collect::<Vec<&str>>().concat();

            tickets.push(Ticket {
                title: String::from(title),
                body: Some(body),
            })
        }

        self.tickets = Some(tickets);
    }

    fn add_task(&mut self, title: String, body: Option<String>, filename: Option<PathBuf>) -> () {
        let clean_body = if let Some(body) = body {
            body
        } else {
            String::new()
        };

        let clean_filename = if let Some(filename) = filename {
            filename
        } else {
            let mut file_path_buf = PathBuf::from(title.clone().to_lowercase().replace(" ", "_"));
            file_path_buf.set_extension("md");
            self.path.join(file_path_buf)
        };

        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(clean_filename)
            .unwrap();

        file.write_all("---\n".as_bytes());
        file.write_all(format!(r#"title:{}"#, title).as_bytes());
        file.write_all("\n".as_bytes());
        file.write_all("---\n".as_bytes());
        file.write_all(clean_body.as_bytes());
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

#[derive(Debug)]
pub struct Ticket {
    title: String,
    body: Option<String>,
}

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
                .trim()
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

fn append_to_store_list(
    path: PathBuf,
    stores: &mut HashMap<PathBuf, Store>,
    config: Config,
) -> io::Result<()> {
    if let Some(_store) = stores.get(&path) {
        // If we already have something, return.
        return Ok(());
    } else {
        let store_lines_path = config.mdpm_share.join(STORE_LINES);
        let mut store_lines_file = fs::OpenOptions::new()
            .append(true)
            .open(&store_lines_path)
            .unwrap();

        let cwd = std::env::current_dir().unwrap();

        store_lines_file
            .write_all(cwd.join(&path).to_str().unwrap().as_bytes())
            .unwrap();
        store_lines_file.write_all("\n".as_bytes()).unwrap();
        stores.insert(path.clone(), Store::from(path.clone()));
    }

    Ok(())
}

pub fn handle_command(
    commands: Option<Commands>,
    stores: &mut HashMap<PathBuf, Store>,
    config: Config,
) -> io::Result<()> {
    if let Some(command) = commands {
        match command {
            Commands::Init => init_store(stores, config),
            Commands::New {
                title,
                body,
                filename,
            } => new_task(title, body, filename, stores, config),
            _ => todo!(),
        }
    } else {
        list_store(stores, config);
    }

    Ok(())
}

fn init_store(stores: &mut HashMap<PathBuf, Store>, config: Config) {
    let dir_to_create = PathBuf::from(".mdpm");

    match fs::create_dir(&dir_to_create) {
        Ok(()) => {
            append_to_store_list(dir_to_create, stores, config).unwrap();
        }
        Err(_) => {
            println!(".mdpm already exists for this directory.")
        }
    }
}

fn new_task(
    title: String,
    body: Option<String>,
    filename: Option<PathBuf>,
    stores: &mut HashMap<PathBuf, Store>,
    _config: Config,
) {
    let store = locate_store(stores).unwrap();
    store.add_task(title, body, filename);
}

fn locate_store(stores: &mut HashMap<PathBuf, Store>) -> Option<&mut Store> {
    let store_dir = std::env::current_dir().unwrap().join(".mdpm");
    let store = stores.get_mut(&store_dir).unwrap();
    Some(store)
}

fn list_store(stores: &mut HashMap<PathBuf, Store>, _config: Config) {
    let store_dir = std::env::current_dir().unwrap().join(".mdpm");
    if let Some(store) = stores.get_mut(&store_dir) {
        store.hydrate();
    } else {
        // list them all!
        for (_path, store) in stores.iter_mut() {
            store.hydrate();
            println!("{:?}", store);
        }
    }
}
