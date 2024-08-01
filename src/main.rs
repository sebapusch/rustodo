mod draw;
mod panel;
mod settings;
mod todo;

use std::fs::{read_dir, File};
use std::io::Read;
use std::{env, fs};

pub use crate::panel::Panel;
pub use crate::settings::Settings;
pub use crate::todo::{Todo, TodoList};

const PANEL: &str = "-p";
const NEW: &str = "-n";
const LIST: &str = "-l";

fn main() {
    let settings = load_settings().unwrap();

    let mut args: Vec<String> = env::args().collect();
    args = args[1..].to_owned();

    match get_command(&args) {
        Ok(cmd) => match cmd {
            PANEL => match open_todo_list(&settings, &args[0].to_string()) {
                Ok(list) => Panel::new(list, settings).start(),
                Err(err) => println!("Unable to start panel: {}", err),
            },
            NEW => match add_todo_list(&settings, args) {
                Ok(list) => Panel::new(list, settings).start(),
                Err(err) => println!("Unable to start panel: {}", err),
            },
            LIST => {
                list_todo_lists(&settings).unwrap();
            }
            _ => {}
        },
        Err(err) => println!("{}", err),
    };
}

fn get_command(args: &Vec<String>) -> Result<&'static str, String> {
    if args.is_empty() {
        return Err(String::from("Please enter a valid command"));
    }

    if args[0].starts_with("-n") {
        return Ok("-n");
    }

    if args[0].starts_with("-l") {
        return Ok("-l");
    }

    return Ok("-p");
}

fn list_todo_lists(settings: &Settings) -> Result<(), String> {
    let path = &settings.todopath;

    let iter = match read_dir(path) {
        Ok(entries) => entries,
        Err(err) => {
            return Err(format!(
                "Unable to read todo lists at path '{}': {}",
                path, err
            ))
        }
    };

    for file in iter {
        let entry = match file {
            Ok(entry) => entry,
            _ => continue,
        };

        let entry_type = entry.file_type().unwrap();

        if !entry_type.is_file() {
            continue;
        }

        let entry_name = entry.file_name().to_str().to_owned().unwrap().to_string();

        let todo_list = match open_todo_list(settings, &entry_name) {
            Ok(todo_list) => todo_list,
            Err(_) => continue,
        };

        println!(
            "{}: {}/{}",
            todo_list.name,
            todo_list.completed(),
            todo_list.total()
        );
    }

    Ok(())
}

fn add_todo_list(settings: &Settings, args: Vec<String>) -> Result<TodoList, String> {
    if args.len() < 2 {
        return Err(String::from("Please provide a valid list name"));
    }

    let todo_list = TodoList {
        name: args[1].to_string(),
        todos: Vec::new(),
    };

    todo_list
        .save(&settings.todopath)
        .expect("Error writing file");

    Ok(todo_list)
}

fn open_todo_list(settings: &Settings, name: &String) -> Result<TodoList, String> {
    let dir_path = &settings.todopath;

    let path = format!("{}/{}.json", &dir_path, &name.replace(".json", ""));

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(err) => {
            return Err(format!(
                "Unable to open todo file at path '{}': {}",
                &path, err
            ))
        }
    };

    let mut data = String::new();

    match file.read_to_string(&mut data) {
        Err(err) => return Err(format!("Unable to read todo file: {}", err)),
        _ => (),
    };

    match serde_json::from_str(data.as_str()) {
        Ok(todo_list) => Ok(todo_list),
        Err(err) => Err(format!("Unable to parse todo list: {}", err)),
    }
}

fn load_settings() -> Result<Settings, String> {
    let settings = match Settings::load() {
        Err(err) => return Err(err),
        Ok(s) => s,
    };

    match fs::create_dir_all(settings.todopath.clone()) {
        Ok(_) => return Ok(settings),
        Err(err) => return Err(format!("Unable to load todo path: {}", err)),
    };
}
