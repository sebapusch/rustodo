use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoList {
    pub name: String,
    pub todos: Vec<Todo>,
}

impl TodoList {
    pub fn new(name: String) -> Self {
        TodoList {
            name,
            todos: Vec::new(),
        }
    }

    pub fn to_json(&self) -> String {
        return serde_json::to_string(self).expect("Error serializing json");
    }

    pub fn save(&self, dir_path: &String) -> Result<(), String> {
        let json = self.to_json();
        let path = format!("{}/{}.json", dir_path, self.name);

        match fs::write(&path, &json) {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }

    pub fn completed(&self) -> usize {
        let mut completed = 0;

        for item in self.todos.to_owned() {
            if item.done {
                completed += 1;
            }
        }

        completed
    }

    pub fn total(&self) -> usize {
        self.todos.len()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
    pub id: i16,
    pub item: String,
    pub priority: i8,
    pub tags: Vec<String>,
    pub done: bool,
}

impl Todo {
    pub fn toggle(&mut self) {
        self.done = !self.done;
    }
}
