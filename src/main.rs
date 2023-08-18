mod todo;
mod panel;
mod draw;
mod settings;

use std::{env, fs};
use std::fs::File;
use std::io::{BufReader, Read};

pub use crate::todo::{TodoList, Todo};
pub use crate::panel::Panel;
pub use crate::settings::Settings;

const PANEL: &str = "-p";
const NEW: &str = "-n";

fn main() {


    let settings = load_settings().unwrap();

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
        }
        _ => {}
    }
}

fn get_command(args: Vec<String>) -> Result<&'static str, String> {

    if args.is_empty() {
        return Err(String::from("Please enter a valid command"));
    }

    if args[0].starts_with("-") {
        return Ok("-n");
    }

    return Ok("-p");
}

//fn add_todo(args: Vec<String>) -> Result<(), String> {
//    if args.is_empty() {
//        return Err(String::from("Please enter a valid todo item"));
//    }

//    let mut list = open_todo_list(String::from("tester"));

//    let todo = Todo {
//        item: args.join(" "),
//        priority: 1,
//        tags: Vec::new(),
//        done: false,
//        id: 1,
//    };

//    list.todos.push(todo);

//    list.save(String::from("/home/sebastianp/todos"))
//        .expect("Unable to save file");

//    println!("Stored todo item");
//    Ok(())
//}

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
    let path = format!("{}/{}.json", todopath, name);
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

//fn list_todo_lists() -> Result<(), String> {
//    println!("list todo lists");
//    Ok(())
//}

