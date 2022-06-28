use colored::*;
use config::{Config, ConfigError};
use csv;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process;

use super::types::{Files, Records, Replace, TableError, Tables, Template};

// From https://stackoverflow.com/a/52367953/16134348
pub fn string_to_sstr(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

pub fn stdout(selector: &str, message: &str) {
    // TODO implement debug level selection
    // TODO implement IO error handling
    match selector {
        "info" => {
            println!(
                "{} {}",
                "[machinegen]".bright_blue().bold(),
                message.bright_blue()
            );
        }
        "fatal" => {
            println!(
                "{} {} {}",
                "[machinegen]".bright_red().bold(),
                "[Fatal]".bright_purple().bold(),
                message.bright_red().bold()
            );
            process::exit(1);
        }
        "error" => {
            println!(
                "{} {}",
                "[machinegen]".bright_red().bold(),
                message.bright_red().bold()
            );
        }
        "warning" => {
            println!("{} {}", "[machinegen]".yellow().bold(), message.yellow());
        }
        "success" => {
            println!(
                "{} {}",
                "[machinegen]".bright_green().bold(),
                message.bright_green()
            );
        }
        _ => {
            println!("{} {}", "[machinegen]".normal().bold(), message.normal());
        }
    }
}

pub fn call_with_stdout(
    exit_code: Result<process::ExitStatus, io::Error>,
    success_message: &str,
    error_message: &str,
) -> bool {
    let exit_code = match exit_code {
        Ok(code) => code,
        Err(error) => {
            stdout("error", &format!("{}", error));
            return false;
        }
    };

    if exit_code.success() {
        stdout("success", success_message);
        return true;
    } else {
        stdout("error", error_message);
        return false;
    }
}

pub fn read_user_config() -> Result<Config, ConfigError> {
    let mut path = PathBuf::new();

    path.push(".machinegen");
    path.push("config");
    path.push("user");
    path.set_extension("json");

    // load and return the config
    let config = Config::builder()
        .add_source(config::File::new("site", config::FileFormat::Json5))
        .build();
    return config;
}

pub fn load_table(table_type: Records) -> Result<Tables, TableError> {
    let mut path = PathBuf::new();

    path.push(".machinegen");
    path.push("config");
    path.push("tables");
    path.push(table_type.name());
    path.set_extension("csv");

    // load and return the config
    let file = fs::read_to_string(path);

    match file {
        Ok(file) => match table_type {
            Records::Files(_) => {
                let mut table: Tables = Vec::new();
                let mut reader = csv::Reader::from_reader(file.as_bytes());
                for record in reader.deserialize::<Files>() {
                    let parsed: Files = match record {
                        Ok(record) => record,
                        Err(error) => {
                            stdout("error", &format!("{}", error));
                            return Err(TableError::Csv(error));
                        }
                    };
                    table.push(Records::Files(parsed));
                }
                return Ok(table);
            }
            Records::Replace(_) => {
                let mut table: Tables = Vec::new();
                let mut reader = csv::Reader::from_reader(file.as_bytes());
                for record in reader.deserialize::<Replace>() {
                    let parsed: Replace = match record {
                        Ok(record) => record,
                        Err(error) => {
                            stdout("error", &format!("{}", error));
                            return Err(TableError::Csv(error));
                        }
                    };
                    table.push(Records::Replace(parsed));
                }
                return Ok(table);
            }
            Records::Template(_) => {
                let mut table: Tables = Vec::new();
                let mut reader = csv::Reader::from_reader(file.as_bytes());
                for record in reader.deserialize::<Template>() {
                    let parsed: Template = match record {
                        Ok(record) => record,
                        Err(error) => {
                            stdout("error", &format!("{}", error));
                            return Err(TableError::Csv(error));
                        }
                    };
                    table.push(Records::Template(parsed));
                }
                return Ok(table);
            }
        },
        Err(error) => {
            return Err(TableError::Io(error));
        }
    }
}
