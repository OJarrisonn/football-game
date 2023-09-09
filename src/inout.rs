use std::{fs::{File, OpenOptions}, error::Error};

use serde::{de::DeserializeOwned, ser::Serialize};

pub fn load_yaml<T>(path: &str) -> Result<T, Box<dyn Error>> 
where T: DeserializeOwned {
    let yaml_file = match File::open(path) {
        Ok(file) => file,
        Err(e) => return Err(Box::new(e)),
    };

    match serde_yaml::from_reader::<File, T>(yaml_file) {
        Ok(yaml) => Ok(yaml),
        Err(e) => Err(Box::new(e)),
    }
}

pub fn save_yaml<T>(path: &str, data: &T) -> Result<(), Box<dyn Error>>
where T: Serialize {
    let yaml_file = OpenOptions::new().create(true).write(true).open(path);

    let yaml_file = match yaml_file {
        Ok(file) => file,
        Err(e) => return Err(Box::new(e)),
    };

    match serde_yaml::to_writer(yaml_file, data) {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}