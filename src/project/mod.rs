use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub author: String,
}

impl Project {
    pub fn new(name: String, description: String, author: String) -> Project {
        Project {
            name, description, author
        }
    }

    pub fn store(self: &Self, file_name: &str) -> std::io::Result<()> {
        let mut project_file = File::create(file_name)?;
        project_file.write_all(serde_json::to_string(self).unwrap().as_bytes())?;
        Ok(())
    }

    pub fn load<P: AsRef<Path>>(file_name: P) -> std::io::Result<Project> {
        let mut project_file = File::open(file_name)?;
        Ok(serde_json::from_reader(&mut project_file)?)
    }
}