mod debug;
mod draw;
mod panel;
mod reader;
mod settings;
mod todo;

use std::fs::{read_dir, File};
use std::io::Read;
use std::{env, fs};

pub use crate::panel::Panel;
pub use crate::settings::Settings;
pub use crate::todo::{Todo, TodoList};

enum Command {
    OpenListPanel(String),
    NewList(String),
    ListLists,
}

fn main() {
    let settings = load_settings().unwrap();

    match parse_command() {
        Ok(cmd) => match cmd {
            Command::ListLists => {
                list_todo_lists(&settings).unwrap_or_else(|err| {
                    println!("{}", err);
                });
            }
            Command::NewList(name) => match create_todo_list(&settings, name) {
                Ok(list) => Panel::new(list, settings).start(),
                Err(err) => println!("{}", err),
            },
            Command::OpenListPanel(name) => match open_todo_list(&settings, name) {
                Ok(list) => Panel::new(list, settings).start(),
                Err(err) => println!("{}", err),
            },
        },
        Err(err) => println!("Unable to parse command: {}", err),
    };
}

fn parse_command() -> Result<Command, String> {
    let mut args: Vec<String> = env::args().collect();
    args = args[1..].to_owned();

    if args.is_empty() {
        return Err("Please enter a valid command".into());
    }

    if args[0] == "new" {
        if args.len() < 2 || args[1].trim().len() == 0 {
            return Err("Please provide a valid list name".into());
        } else {
            return Ok(Command::NewList(args[1].trim().to_string()));
        }
    }

    if args[0] == "list" {
        Ok(Command::ListLists)
    } else {
        Ok(Command::OpenListPanel(args[0].trim().into()))
    }
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

        let todo_list = match open_todo_list(settings, entry_name.to_string()) {
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

fn create_todo_list(settings: &Settings, name: String) -> Result<TodoList, String> {
    let created_list = TodoList::new(name);

    match created_list.save(&settings.todopath) {
        Ok(_) => Ok(created_list),
        Err(err) => Err(err),
    }
}

fn open_todo_list(settings: &Settings, name: String) -> Result<TodoList, String> {
    let path = format!("{}/{}.json", &settings.todopath, &name.replace(".json", ""));

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
