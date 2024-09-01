use crate::config::Config;
use crate::config::I3Switcher;
use crate::config::WorkspaceSwitcher;
use crate::input::Items;
use anyhow::anyhow;
use std::io::Write;
use std::process::Command;
use std::process::Output;
use std::process::Stdio;

pub trait ExecSwitcher {
    fn exec(&self) -> Option<Result<Output, std::io::Error>>;
}

impl ExecSwitcher for WorkspaceSwitcher {
    fn exec(&self) -> Option<Result<Output, std::io::Error>> {
        if let Some(command) = &self.custom {
            command.exec()
        } else if let Some(i3_swicth) = &self.i3 {
            println!("Yeag");
            i3_swicth.exec()
        } else {
            None
        }
    }
}

impl ExecSwitcher for String {
    fn exec(&self) -> Option<Result<Output, std::io::Error>> {
        let values: Vec<String> = self.split(" ").map(|s| s.to_string()).collect();
        let comm = Command::new(values.first()?)
            .args(values[1..].iter())
            .output();
        Some(comm)
    }
}

impl ExecSwitcher for I3Switcher {
    fn exec(&self) -> Option<Result<Output, std::io::Error>> {
        Some(
            Command::new("i3-msg")
                .args([
                    "workspace",
                    "number",
                    self.workspace_number.to_string().as_str(),
                ])
                .output(),
        )
    }
}

pub fn launch_rofi(items: &Items, config: &Config) -> anyhow::Result<Option<String>> {
    let names = items.get_names();

    let mut child = Command::new("rofi")
        .args(["-dmenu", "-i", "-theme", config.theme.as_str()])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let mut stdin = child.stdin.take().ok_or(anyhow!("Cannot open stdin"))?;
    std::thread::spawn(move || {
        stdin
            .write_all(names.as_bytes())
            .expect("Failed to write to stdin");
    });
    let output = child.wait_with_output()?;

    if output.status.success() {
        let output_string = std::str::from_utf8(&output.stdout)?;
        if output_string.trim().is_empty() {
            Err(anyhow!("Empty selection"))
        } else {
            Ok(Some(String::from(output_string)))
        }
    } else {
        Ok(None)
    }
}

pub fn launch_link(s: &str, items: &Items, config: &Config) -> anyhow::Result<()> {
    if let Some(switcher) = &config.workspace_switcher {
        println!("yep");
        if let Some(output) = switcher.exec() {
            if !output?.status.success() {
                return Err(anyhow!("i3-msg failed"));
            }
        }
    }

    let link = items.get_link(s.trim()).ok_or(anyhow!("Invalid item"))?;
    Command::new(config.browser_command_name.as_str())
        .args([link])
        .output()?;

    Ok(())
}
