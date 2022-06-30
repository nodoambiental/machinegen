use colored::*;
use config::{Config, ConfigError};
use csv;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::{env, fs, io, process};

use crate::types::{ConfigEntry, ConfigPrimitives, FilesEntry, ReplaceEntry, TemplateEntry};

use super::types::{
    Files, MachineData, Records, Replace, TableError, TableTypes, Tables, Template,
};

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
        "debug" => {
            println!(
                "{}{} {}",
                "[machinegen]".bright_purple().bold(),
                "[debug]".yellow().bold(),
                message.italic().yellow()
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

pub fn process_relations() -> Result<MachineData, TableError> {
    #[cfg(feature = "debug")]
    stdout(
        "debug",
        format!("Start process_relations function").as_str(),
    );
    //
    //      Load tables as their correct types
    //
    let replace_table = match load_table(TableTypes::Replace) {
        Ok(table) => {
            let mut parsed_table: Vec<Replace> = Vec::new();
            for record in table {
                match record {
                    Records::Replace(replace_record) => parsed_table.push(replace_record),
                    _ => {
                        stdout(
                            "fatal",
                            "Error reading Replace table. It appears to be mislabeled as such. Aborting.",
                        );
                        unreachable!("Program should be aborted by fatal statement above.");
                    }
                }
            }
            parsed_table
        }
        Err(error) => {
            stdout(
                "error",
                &format!("Error reading Replace table: {:?}", error),
            );
            return Err(error);
        }
    };
    let files_table = match load_table(TableTypes::Files) {
        Ok(table) => {
            let mut parsed_table: Vec<Files> = Vec::new();
            for record in table {
                match record {
                    Records::Files(files_record) => parsed_table.push(files_record),
                    _ => {
                        stdout(
                            "fatal",
                            "Error reading Files table. It appears to be mislabeled as such. Aborting.",
                        );
                        unreachable!("Program should be aborted by fatal statement above.");
                    }
                }
            }
            parsed_table
        }
        Err(error) => {
            stdout("error", &format!("Error reading Files table: {:?}", error));
            return Err(error);
        }
    };
    let template_table = match load_table(TableTypes::Template) {
        Ok(table) => {
            let mut parsed_table: Vec<Template> = Vec::new();
            for record in table {
                match record {
                    Records::Template(template_record) => parsed_table.push(template_record),
                    _ => {
                        stdout(
                            "fatal",
                            "Error reading Templates table. It appears to be mislabeled as such. Aborting.",
                        );
                        unreachable!("Program should be aborted by fatal statement above.");
                    }
                }
            }
            parsed_table
        }
        Err(error) => {
            stdout(
                "error",
                &format!("Error reading Templates table: {:?}", error),
            );
            return Err(error);
        }
    };

    #[cfg(feature = "debug")]
    stdout("debug", format!("Tables loaded correctly\n").as_str());
    stdout(
        "debug",
        format!("Building the replacements struct...").as_str(),
    );

    //
    //      Define needed stuff to build the machine data fields
    //

    let mut config_keys: HashSet<String> = HashSet::new();
    let mut config_groups: HashMap<String, HashSet<String>> = HashMap::new();

    let mut files_keys: HashMap<String, HashSet<String>> = HashMap::new();

    let mut template_keys: HashSet<String> = HashSet::new();

    let mut replacements: HashMap<String, ReplaceEntry> = HashMap::new();
    let mut files: HashMap<String, FilesEntry> = HashMap::new();
    let mut templates: HashMap<String, TemplateEntry> = HashMap::new();

    //
    //      Get both the available user config keys and the replacements struct
    //

    for record in replace_table {
        if record.config_parent.as_str() != "root" {
            match config_groups.get_mut(&record.config_parent) {
                Some(group) => {
                    group.insert(record.string.clone());
                }
                None => {
                    let mut group: HashSet<String> = HashSet::new();
                    group.insert(record.string.clone());
                    config_groups.insert(record.config_parent.clone(), group);
                }
            }
        } else {
            config_keys.insert(record.string.clone());
        }
        match template_keys.get(&record.template) {
            Some(_) => {}
            None => {
                template_keys.insert(record.template.clone());
            }
        }

        replacements.insert(
            record.string,
            ReplaceEntry {
                description: record.description,
                mandatory: record.mandatory,
                unique: record.unique,
                template: record.template,
                config_parent: record.config_parent,
            },
        );
    }

    #[cfg(feature = "debug")]
    stdout(
        "debug",
        format!(
            "Done. Replacements Struct:\n {:?} \n Building files struct...\n",
            replacements
        )
        .as_str(),
    );

    //
    //     Get the files struct
    //

    for record in files_table {
        if record.config_parent.as_str() != "root" {
            match config_groups.get_mut(&record.config_parent) {
                Some(group) => match group.get("files") {
                    Some(_) => {}
                    None => {
                        group.insert(String::from("files"));
                    }
                },
                None => {
                    let mut group: HashSet<String> = HashSet::new();
                    group.insert(record.name.clone());
                    config_groups.insert(record.config_parent.clone(), group);
                    config_groups
                        .get_mut(&record.config_parent)
                        .unwrap()
                        .insert(String::from("files"));
                }
            }
        } else {
            if !config_keys.contains("files") {
                config_keys.insert(String::from("files"));
            }
        }

        if !files_keys.contains_key(record.config_parent.as_str()) {
            let mut files: HashSet<String> = HashSet::new();
            files.insert(record.name.clone());
            files_keys.insert(record.config_parent.clone(), files);
        } else {
            match files_keys.get_mut(record.config_parent.as_str()) {
                Some(files) => {
                    files.insert(record.name.clone());
                }
                None => {
                    stdout(
                        "fatal",
                        concat!("Error processing file config groups. It appears there was some corruption in the process.\n",
                    "Open a issue on Github if this problem repeats."),
                    );
                    unreachable!("Program should be aborted by fatal statement above.");
                }
            }
        }

        files.insert(
            record.name,
            FilesEntry {
                system: record.system,
                target: record.target,
                description: record.description,
            },
        );
    }

    #[cfg(feature = "debug")]
    stdout(
        "debug",
        format!(
            "Done. Files Struct:\n {:?} \n Building templates struct...\n",
            files
        )
        .as_str(),
    );

    //
    //      Get the templates struct
    //

    for record in template_table {
        let mut template_entries: HashMap<String, ReplaceEntry> = HashMap::new();

        //   This decidedly not performant approach will filter the replacements for this template
        for entry in &replacements {
            if entry.1.template == record.name {
                template_entries.insert(entry.0.clone(), entry.1.clone());
            }
        }

        templates.insert(
            record.name.clone(),
            TemplateEntry {
                system: record.system,
                source: record.source,
                target: record.target,
                description: record.description,
                replacements: template_entries,
            },
        );
    }

    #[cfg(feature = "debug")]
    stdout(
        "debug",
        format!(
            "Done. Templates Struct:\n {:?} \n Building configuration struct...\n",
            templates
        )
        .as_str(),
    );

    let mut config_entries: HashMap<String, ConfigEntry> = HashMap::new();

    let mut all_config_keys: HashSet<String> = HashSet::new();
    all_config_keys.extend(config_keys.clone());
    all_config_keys.extend(config_groups.keys().cloned().collect::<HashSet<String>>());

    for key in all_config_keys {
        if config_groups.contains_key(&key) {
            let mut children: HashMap<String, ConfigEntry> = HashMap::new();
            let group = match config_groups.get(&key) {
                Some(group) => group,
                None => {
                    stdout(
                        "fatal",
                        concat!("Error reading config groups. It appears there was some corruption in the process.\n",
                        "Open a issue on Github if this problem repeats."),
                    );
                    unreachable!("Program should be aborted by fatal statement above.");
                }
            };

            for child in group {
                if child == &"files" {
                    let mut files_children: HashMap<String, ConfigEntry> = HashMap::new();

                    let file_list = match files_keys.get(&key) {
                        Some(hashset) => hashset,
                        None => {
                            stdout(
                            "fatal",
                            concat!("Error brocessing files. It appears there was some corruption in the process.\n",
                            "Open a issue on Github if this problem repeats."),
                        );
                            unreachable!("Program should be aborted by fatal statement above.");
                        }
                    };

                    for current_file in file_list {
                        let file_data = match files.get(current_file) {
                            Some(entry) => entry,
                            None => {
                                stdout(
                                "fatal",
                                concat!("Error processing files. It appears there was some corruption in the process.\n",
                                "Open a issue on Github if this problem repeats."),
                            );
                                unreachable!("Program should be aborted by fatal statement above.");
                            }
                        };
                        files_children.insert(
                            String::from(current_file),
                            ConfigEntry {
                                children: None,
                                description: file_data.description.clone(),
                                mandatory: true,
                                unique: true,
                                value: Some(ConfigPrimitives::NoValue),
                            },
                        );
                    }

                    children.insert(String::from("files"), ConfigEntry {
                        children: Some(files_children),
                        description: String::from("Set of file paths to be copied from according to the machine data specs."),
                        mandatory: true,
                        unique: true,
                        value: None
                    });
                } else {
                    let replacement = match replacements.get(child) {
                        Some(entry) => entry,
                        None => {
                            stdout(
                            "fatal",
                            concat!("Error reading replacements. It appears there was some corruption in the process.\n",
                            "Open a issue on Github if this problem repeats."),
                        );
                            unreachable!("Program should be aborted by fatal statement above.");
                        }
                    };

                    children.insert(
                        String::from(child.clone()),
                        ConfigEntry {
                            children: None,
                            description: replacement.description.clone(),
                            mandatory: replacement.mandatory,
                            unique: replacement.unique,
                            value: Some(if replacement.unique {
                                ConfigPrimitives::NoValue
                            } else {
                                ConfigPrimitives::Array
                            }),
                        },
                    );
                }
            }
            config_entries.insert(
                key,
                ConfigEntry {
                    children: Some(children),
                    description: String::from("Group of config entries"),
                    unique: true,
                    mandatory: true,
                    value: None,
                },
            );
        } else {
            if &key.as_str() == &"files" {
                let mut files_children: HashMap<String, ConfigEntry> = HashMap::new();

                let file_list = match files_keys.get("root") {
                    Some(hashset) => hashset,
                    None => {
                        stdout(
                            "fatal",
                            concat!("Error brocessing files. It appears there was some corruption in the process.\n",
                            "Open a issue on Github if this problem repeats."),
                        );
                        unreachable!("Program should be aborted by fatal statement above.");
                    }
                };

                for current_file in file_list {
                    let file_data = match files.get(current_file) {
                        Some(entry) => entry,
                        None => {
                            stdout(
                                "fatal",
                                concat!("Error processing files. It appears there was some corruption in the process.\n",
                                "Open a issue on Github if this problem repeats."),
                            );
                            unreachable!("Program should be aborted by fatal statement above.");
                        }
                    };
                    files_children.insert(
                        String::from(current_file),
                        ConfigEntry {
                            children: None,
                            description: file_data.description.clone(),
                            mandatory: true,
                            unique: true,
                            value: Some(ConfigPrimitives::NoValue),
                        },
                    );
                }

                config_entries.insert(String::from("files"), ConfigEntry {
                    children: Some(files_children),
                    description: String::from("Set of file paths to be copied from according to the machine data specs."),
                    mandatory: true,
                    unique: true,
                    value: None
                });
            } else {
                let replacement = match replacements.get(&key) {
                    Some(entry) => entry,
                    None => {
                        stdout(
                            "fatal",
                            &format!("Config key {} is not in the replacements table. Should be in the config_parent column.", key),
                        );
                        unreachable!("Program should be aborted by fatal statement above.");
                    }
                };
                config_entries.insert(
                    key,
                    ConfigEntry {
                        children: None,
                        description: replacement.description.clone(),
                        unique: replacement.unique.clone(),
                        mandatory: replacement.mandatory,
                        value: Some(if replacement.unique {
                            ConfigPrimitives::NoValue
                        } else {
                            ConfigPrimitives::Array
                        }),
                    },
                );
            }
        }
    }

    #[cfg(feature = "debug")]
    stdout(
        "debug",
        format!(
            "Done. Configuration Struct:\n {:?} \n Table processing complete.\n",
            config_entries
        )
        .as_str(),
    );

    Ok(MachineData {
        files: files,
        templates: templates,
        config_keys: config_entries,
    })
}

pub fn cwd_string() -> String {
    env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap()
}

pub fn load_table(table_type: TableTypes) -> Result<Tables, TableError> {
    let mut path = PathBuf::new();

    path.push(cwd_string());
    path.push(".machinegen");
    path.push("config");
    path.push("tables");
    path.push(table_type.name());
    path.set_extension("csv");

    #[cfg(feature = "debug")]
    stdout(
        "debug",
        format!(
            "Loading table {}\n located at {}",
            table_type.name(),
            path.to_str().unwrap()
        )
        .as_str(),
    );

    // load and return the config
    let file = fs::read_to_string(path);

    match file {
        Ok(file) => match table_type {
            TableTypes::Files => {
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
                #[cfg(feature = "debug")]
                stdout(
                    "debug",
                    format!("Table {} loaded correctly.\n", table_type.name()).as_str(),
                );
                return Ok(table);
            }
            TableTypes::Replace => {
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
                #[cfg(feature = "debug")]
                stdout(
                    "debug",
                    format!("Table {} loaded correctly.\n", table_type.name()).as_str(),
                );
                return Ok(table);
            }
            TableTypes::Template => {
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
                    // ? Why the heck this prints again three times?
                    #[cfg(feature = "debug")]
                    stdout(
                        "debug",
                        format!("Table {} loaded correctly.\n", table_type.name()).as_str(),
                    );
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
