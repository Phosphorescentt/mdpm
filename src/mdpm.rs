use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use crate::cli::{Commands, NewCommands};

pub fn handle_command(command: Commands) -> () {
    match command {
        Commands::Init { store_name } => init_store(store_name),
        Commands::New(new_commands) => match new_commands {
            NewCommands::Project { title, slug } => new_project(title, slug),
            NewCommands::Task { title, project } => new_task(title, project),
        },
    }
}

fn init_store(store_name: String) -> () {
    // First create the metadata file.
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

    // Then create the projects and the tasks directory.
    let _tasks_dir = fs::create_dir("tasks/").unwrap();
    let _projects_dir = fs::create_dir("projects/").unwrap();

    // Create the default EMPTY project.
    new_project("Empty Project".to_string(), "EMPT".to_string());
}

fn new_task(title: String, project_slug: Option<String>) -> () {
    let validated_project_slug: String = match project_slug {
        Some(p) => {
            if project_exists(p.clone()) {
                p
            } else {
                panic!("Unable to find project.")
            }
        }
        None => "EMPT".to_string(),
    };

    let file_id = new_id_for_project(&validated_project_slug);
    let slug = format!("{}-{}", validated_project_slug, file_id);
    let mut task_file = File::create_new(format!("tasks/{}.task.md", slug)).unwrap();

    task_file.write_all(
        format!(
            "title: {}\nslug: {}\nproject_slug: {}\n",
            title, slug, validated_project_slug
        )
        .as_bytes(),
    );
}

fn new_project(title: String, slug: String) -> () {
    let mut project_file = File::create_new(format!("projects/{}.project.md", slug)).unwrap();
    project_file
        .write_all(format!("title: {}\nslug: {}\n--- \n", title, slug).as_bytes())
        .unwrap();

    println!("Created project {} with slug {}", title, slug);
}

fn project_exists(project_slug: String) -> bool {
    Path::new(&format!("projects/{}.project.md", project_slug)).exists()
}

fn new_id_for_project(project_slug: &String) -> u32 {
    let paths = fs::read_dir("tasks/").unwrap();
    let max_id = paths
        .filter_map(|x| {
            let current_file_name = x
                .as_ref()
                .unwrap()
                .file_name()
                .to_string_lossy()
                .to_string();
            if current_file_name.starts_with(project_slug) {
                let (_filename, remainder) = current_file_name.split_once("-").unwrap();
                let (id, _extension) = remainder.split_once(".").unwrap();
                Some(id.parse::<u32>().unwrap())
            } else {
                None
            }
        })
        .max();

    match max_id {
        Some(id) => return id + 1,
        None => return 1,
    }
}
