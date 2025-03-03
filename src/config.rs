use std::{
    fs::{create_dir_all, File},
    io::{Read, Write},
};

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub input_file: String,
    pub theme: String,
    pub separator: char,
    pub browser_command_name: String,
    pub workspace_switcher: Option<WorkspaceSwitcher>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WorkspaceSwitcher {
    pub custom: Option<String>,
    pub i3: Option<I3Switcher>,
}

impl From<I3Switcher> for WorkspaceSwitcher {
    fn from(value: I3Switcher) -> Self {
        WorkspaceSwitcher {
            custom: None,
            i3: Some(value),
        }
    }
}

impl From<String> for WorkspaceSwitcher {
    fn from(value: String) -> Self {
        WorkspaceSwitcher {
            custom: Some(value),
            i3: None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct I3Switcher {
    pub workspace_number: u8,
}

impl Default for I3Switcher {
    fn default() -> Self {
        I3Switcher {
            workspace_number: 1,
        }
    }
}

impl I3Switcher {
    pub fn new(number: u8) -> Self {
        Self {
            workspace_number: number,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            input_file: format!("{}/links.txt", config_folder_path()),
            theme: format!("{}/theme.rasi", config_folder_path()),
            separator: ',',
            browser_command_name: String::from("firefox"),
            workspace_switcher: Some(I3Switcher::default().into()),
        }
    }
}

fn save_yml_config(config: &Config, path: &str) -> anyhow::Result<()> {
    let yaml_string = serde_yaml::to_string(&config)?;

    let mut file = File::create(path)?;
    file.write_all(yaml_string.as_bytes())?;

    Ok(())
}

fn save_default_config(config_path: &str) -> anyhow::Result<Config> {
    let config = Config::default();
    save_yml_config(&config, config_path)?;
    Ok(config)
}

pub fn get_configuration() -> anyhow::Result<Config> {
    let config_path = format!("{}/config.yml", config_folder_path());

    create_dir_all(config_folder_path())?;

    if let Ok(mut file) = File::open(config_path.clone()) {
        let mut string_value = String::new();
        file.read_to_string(&mut string_value)?;
        if string_value.is_empty() {
            save_default_config(config_path.as_str())
        } else {
            Ok(serde_yaml::from_str(&string_value)?)
        }
    } else {
        save_default_config(config_path.as_str())
    }
}

fn config_folder_path() -> String {
    format!(
        "{}/.config/quick-links",
        home::home_dir().unwrap().to_str().unwrap()
    )
}
