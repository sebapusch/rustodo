use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoList {
    pub name: String,
    pub todos: Vec<Todo>,
}

impl TodoList {
    pub fn to_json(&self) -> String {
        return serde_json::to_string(self).expect("Error serializing json");
    }

    pub fn save(&self, dir_path: String) -> std::io::Result<()> {
        let json = self.to_json();
        // todo clean path
        let path = format!("{}/{}.json", dir_path, self.name);

        fs::write(&path, &json)
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
