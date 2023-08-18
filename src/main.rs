mod todo;
mod panel;
mod draw;
mod settings;

use std::{env, fs};
use std::fs::{File, read_dir};
use std::io::{BufReader, Read};

pub use crate::todo::{TodoList, Todo};
pub use crate::panel::Panel;
pub use crate::settings::Settings;

const PANEL: &str = "-p";
const NEW: &str = "-n";
const LIST: &str = "-l";

fn main() {


    let settings = load_settings().unwrap();

    if false {

        list_todo_lists(settings.todopath.clone()).unwrap();
        return;
    }

    let mut args: Vec<String> = env::args().collect();
    args = args[1..].to_owned();

    let command = get_command(args.clone());

    match command.unwrap() {
        PANEL => {
            let list = open_todo_list(args[0].to_string(), settings.todopath.clone());
            let mut panel = Panel::new(list, settings);

            panel.start();
        },
        NEW => {
            let list = add_todo_list(args, settings.todopath.clone())
                .expect("Error creating list");

                let mut panel = Panel::new(list, settings);

            panel.start();
        },
        LIST => {
            list_todo_lists(settings.todopath.to_owned()).unwrap();
        },
        _ => {}
    }
}

fn get_command(args: Vec<String>) -> Result<&'static str, String> {

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

fn list_todo_lists(todopath: String) -> Result<(), String> {
    
    let iter = match read_dir(todopath.clone()) {
        Ok(entries) => entries,
        Err(err) => return Err(format!("Unable to read todo lists at path '{}': {}", todopath, err))
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

        let todo_list = open_todo_list(entry.file_name().to_str().to_owned().unwrap().to_string(), todopath.to_owned());

        println!("{}: {}/{}", todo_list.name, todo_list.completed(), todo_list.total());
    }
 
    Ok(())

}

fn add_todo_list(args: Vec<String>, todopath: String) -> Result<TodoList, String> {

    if args.len() < 2 {
        return Err("Please provide a valid list name".to_string());
    }

    let todo_list = TodoList {
        name: args[1].to_string(),
        todos: Vec::new(),
    };

    todo_list.save(todopath)
        .expect("Error writing file");

    Ok(todo_list)
    
}

fn open_todo_list(name: String, todopath: String) -> TodoList {
    let path = format!("{}/{}.json", todopath, name.replace(".json", ""));
    let mut data = String::new();


    let file = File::open(path.clone()).expect(format!("Unable to open file at path '{:?}'", path.to_owned()).as_str());
    let mut br = BufReader::new(file);

    br.read_to_string(&mut data).expect("Unable to read string");

    serde_json::from_str(&data).expect("Error parsing todo list")
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


