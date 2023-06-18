use std::{env, fs};
use std::fs::File;
use std::io::{BufReader, Read};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct TodoList {
    name: String,
    todos: Vec<Todo>,
}

impl TodoList {
    fn to_json(&self) -> String {
        return serde_json::to_string(self).expect("Error serializing json");
    }

    fn save(&self, dir_path: String) -> std::io::Result<()> {
        let json = self.to_json();
        // todo clean path
        let path = format!("{}/{}.json", dir_path, self.name);

        fs::write(&path, &json)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: i16,
    item: String,
    priority: i8,
    tags: Vec<String>,
    done: bool,
}

impl Todo {
    fn print(&self) {
        let item: String;

        if self.done {
            item = format!("[x] {}", self.item);
        } else {
            item = format!("[ ] {}", self.item);
        }

        println!("{}", item);
    }
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();
    let command = &get_command(args.clone())[..];

    let res = match command {
        "-a" => add_todo(args),
        "-n" => add_todo_list(args),
        "-lt" => list_todos(),
        _ => Err(format!("Unknown command {}", command)),
    };

    res.expect("Error");

    Ok(())
}

fn get_command(args: Vec<String>) -> String {
    if args.is_empty() {
        return "-lt".to_string();
    }

    if args[0].starts_with("-") {
        return args[0].to_string();
    }

    return "-a".to_string();
}

fn list_todos() -> Result<(), String> {
    let list = open_todo_list(String::from("tester"));

    for item in list.todos {
        item.print();
    }

    Ok(())
}

fn add_todo(args: Vec<String>) -> Result<(), String> {
    if args.is_empty() {
        return Err(String::from("Please enter a valid todo item"));
    }

    let mut list = open_todo_list(String::from("tester"));

    let todo = Todo {
        item: args.join(" "),
        priority: 1,
        tags: Vec::new(),
        done: false,
        id: 1,
    };

    list.todos.push(todo);

    list.save(String::from("./stuff"))
        .expect("Unable to save file");

    println!("Stored todo item");
    Ok(())
}

fn add_todo_list(args: Vec<String>) -> Result<(), String> {

    if !args.len() < 2 {
        return Err("Please provide a valid list name".to_string());
    }

    let todo_list = TodoList {
        name: args[1].to_string(),
        todos: Vec::new(),
    };

    todo_list.save(String::from("./stuff"))
        .expect("Error writing file");

    Ok(())
}

fn open_todo_list(name: String) -> TodoList {
    let path = format!("./stuff/{}.json", name);
    let mut data = String::new();

    let file = File::open(path).expect("Unable to open file");
    let mut br = BufReader::new(file);

    br.read_to_string(&mut data).expect("Unable to read string");

    serde_json::from_str(&data).expect("Error parsing todo list")
}

fn list_todo_lists() -> Result<(), String> {
    println!("list todo lists");
    Ok(())
}




