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
    pub workspace_number: u8,
    pub browser_command_name: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            input_file: format!(
                "{}/.config/quick-links/links.txt",
                home::home_dir().unwrap().to_str().unwrap()
            ),

            theme: format!(
                "{}/.config/quick-links/theme.rasi",
                home::home_dir().unwrap().to_str().unwrap()
            ),
            separator: ',',
            workspace_number: 1,
            browser_command_name: String::from("firefox"),
        }
    }
}

fn save_yml_config(config: &Config, path: &str) -> anyhow::Result<()> {
    let yaml_string = serde_yaml::to_string(&config)?;

    let mut file = File::create(path)?;
    file.write_all(yaml_string.as_bytes())?;

    Ok(())
}

pub fn get_configuration() -> anyhow::Result<Config> {
    let config_folder = format!(
        "{}/.config/quick-links",
        home::home_dir().unwrap().to_str().unwrap()
    );

    let config_path = format!("{config_folder}/config.yml");

    create_dir_all(config_folder)?;

    if let Ok(mut file) = File::open(config_path.clone()) {
        let mut string_value = String::new();
        file.read_to_string(&mut string_value)?;
        if string_value.is_empty() {
            let config = Config::default();
            save_yml_config(&config, config_path.as_str())?;
            Ok(config)
        } else {
            Ok(serde_yaml::from_str(&string_value)?)
        }
    } else {
        let config = Config::default();
        save_yml_config(&config, config_path.as_str())?;
        Ok(config)
    }
}
