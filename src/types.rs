use csv;
use serde::Deserialize;
use std::io;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct Replace {
    string: String,
    template: String,
    mandatory: bool,
    unique: bool,
    config_parent: String,
    description: String,
}

#[derive(Deserialize)]
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

#[derive(Deserialize)]
pub struct Template {
    name: String,
    system: System,
    source: PathBuf,
    target: PathBuf,
    description: String,
}

#[derive(Deserialize)]
pub struct Files {
    name: String,
    system: System,
    config_parent: String,
    target: PathBuf,
    description: String,
}

pub enum TableError {
    Io(io::Error),
    Csv(csv::Error),
}

#[derive(Deserialize)]
pub enum Records {
    Replace(Replace),
    Template(Template),
    Files(Files),
}

pub type Tables = Vec<Records>;

impl Records {
    pub fn name(&self) -> &'static str {
        match self {
            Records::Files(_) => "files",
            Records::Replace(_) => "replace",
            Records::Template(_) => "template",
        }
    }
}
