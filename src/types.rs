use csv;
use serde::Deserialize;
use std::io;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct Replace {
    string: String,
    template: String,
    mandatory: bool,
    unique: bool,
    config_parent: String,
    description: String,
}

#[derive(Deserialize, Debug)]
pub enum System {
    Guest,
    Host,
}

impl System {
    pub fn value(&self) -> &'static str {
        match self {
            System::Guest => "guest",
            System::Host => "host",
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Template {
    name: String,
    system: System,
    source: PathBuf,
    target: PathBuf,
    description: String,
}

#[derive(Deserialize, Debug)]
pub struct Files {
    name: String,
    system: System,
    config_parent: String,
    target: PathBuf,
    description: String,
}
#[derive(Debug)]
pub enum TableError {
    Io(io::Error),
    Csv(csv::Error),
}

#[derive(Deserialize, Debug)]
pub enum Records {
    Replace(Replace),
    Template(Template),
    Files(Files),
}

pub enum TableTypes {
    Replace,
    Template,
    Files,
}

pub type Tables = Vec<Records>;

impl TableTypes {
    pub fn name(&self) -> &'static str {
        match self {
            TableTypes::Files => "files",
            TableTypes::Replace => "replace",
            TableTypes::Template => "templates",
        }
    }
}
