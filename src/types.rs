use csv;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::io;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct Replace {
    pub string: String,
    pub template: String,
    pub mandatory: bool,
    pub unique: bool,
    pub config_parent: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct ReplaceEntry {
    pub template: String,
    pub mandatory: bool,
    pub unique: bool,
    pub config_parent: String,
    pub description: String,
}

#[derive(Deserialize, Debug, Clone)]
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
    pub name: String,
    pub system: System,
    pub source: PathBuf,
    pub target: PathBuf,
    pub description: String,
}

#[derive(Debug)]
pub struct TemplateEntry {
    pub system: System,
    pub source: PathBuf,
    pub target: PathBuf,
    pub description: String,
    pub replacements: HashMap<String, ReplaceEntry>,
}

#[derive(Deserialize, Debug)]
pub struct Files {
    pub name: String,
    pub system: System,
    pub config_parent: String,
    pub target: PathBuf,
    pub description: String,
}

#[derive(Debug)]
pub struct FilesEntry {
    pub system: System,
    pub target: PathBuf,
    pub description: String,
}

#[derive(Debug)]
pub enum TableError {
    Io(io::Error),
    Csv(csv::Error),
    Parsing(ParsingError),
}

#[derive(Debug)]
pub struct ParsingError {
    pub message: String,
    pub cause: String,
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

#[derive(Debug)]
pub struct ConfigEntry {
    pub children: Option<HashMap<String, ConfigEntry>>, // if children is Some then value should be None
    pub description: String,
    pub mandatory: bool,
    pub unique: bool,
    pub value: Option<ConfigPrimitives>, // if value is Some then children should be None
}

#[derive(Debug)]
pub enum ConfigPrimitives {
    String,
    i32,
    i64,
    u32,
    u64,
    f32,
    f64,
    bool,
    NoValue,
    Array,
}

pub type NoValue = String;
pub type Array = Vec<ConfigPrimitives>;

#[derive(Debug)]
pub struct MachineData {
    pub config_keys: HashMap<String, ConfigEntry>,
    pub templates: HashMap<String, TemplateEntry>,
    pub files: HashMap<String, FilesEntry>,
}
