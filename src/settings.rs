use std::{fs::{File, self}, io::{Read, Write}};

use home::home_dir;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub todopath: String,
    pub checked_symbol: String,
    pub unchecked_symbol: String,
}

impl Settings {
    pub fn load() -> Result<Settings, String> {


        let homedir = match home_dir() {
            Some(path) => path.display().to_string(),
            None => return Err(String::from("Unable to load home path")),
        };

        let config_path = format!("{}/.config/todo/config.json", homedir);

        match fs::metadata(config_path.clone()) {
            Ok(_) => return Settings::load_existing(config_path),
            _ => ()
        };
        

        match fs::create_dir_all(format!("{}/.config/todo", homedir)) {
            Err(err) => return Err(format!("Unable to create config folder: {}", err)),
            _ => (),
        };

        let settings = Settings::default(homedir);
 
        let mut config = match File::create(config_path.clone()) {
            Err(err) => return Err(format!("Unable to create configuration file at path '{}': {}", config_path, err)),
            Ok(file) => file,
        };

        match config.write_all(settings.as_json().unwrap().as_bytes()) {
            Ok(_) => return Ok(settings),
            Err(err) => return Err(format!("Unable to write to configuration file: {}", err))
        };
        
    }

    fn as_json(&self) -> Result<String, String> {

        match serde_json::to_string(self) {
            Ok(res) => return Ok(res),
            Err(err) => return Err(format!("Unable to serialize configuration: {}", err))
        };
    }

    fn load_existing(path: String) -> Result<Settings, String> {
       let mut file = match File::open(path) {
           Ok(file) => file,
           Err(err) => return Err(format!("Unable to open configuration file: {}", err))
       };


       let mut data = String::new();

       match file.read_to_string(&mut data) {
           Err(err) => return Err(format!("Unable to read configuration file: {}", err)),
           _ => (),
       };

       match serde_json::from_str(data.as_str()) {
           Ok(settings) => Ok(settings),
           Err(err) => Err(format!("Unable to parse configuration: {}", err)),
       }
    }

    fn default(home_path: String) -> Settings {
        
        Settings{
            todopath: format!("{}/todos", home_path),
            checked_symbol: String::from("[x]"),
            unchecked_symbol: String::from("[ ]"),
        } 
    }
} 
