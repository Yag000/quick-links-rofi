use clap_derive::Parser;

use crate::config::Config;

#[derive(Parser)]
pub struct Cli {
    #[clap(short, long)]
    pub input_file: Option<String>,

    #[clap(short, long)]
    pub theme_path: Option<String>,

    #[clap(short, long)]
    pub separator: Option<char>,

    #[clap(short, long)]
    pub browser: Option<String>,

    #[clap(short, long)]
    pub workspace: Option<u8>,
}

impl Cli {
    pub fn update_config(self, mut config: Config) -> Config {
        if let Some(input_file) = self.input_file {
            config.input_file = input_file;
        }
        if let Some(theme_path) = self.theme_path {
            config.theme = theme_path
        }
        if let Some(separator) = self.separator {
            config.separator = separator
        }

        if let Some(browser) = self.browser {
            config.browser_command_name = browser
        }

        if let Some(workspace) = self.workspace {
            config.workspace_number = workspace
        }

        config
    }
}
