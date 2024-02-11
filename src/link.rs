use crate::config::Config;
use crate::input::Item;
use crate::input::Items;
use anyhow::anyhow;
use std::io::Write;
use std::process::Command;
use std::process::Stdio;

pub fn item_to_command(item: &Item) -> String {
    format!("i3-msg workspace number 1 && firefox \"{}\"", item.link)
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
    let command = Command::new("i3-msg")
        .args([
            "workspace",
            "number",
            config.workspace_number.to_string().as_str(),
        ])
        .output()?;

    if command.status.success() {
        let link = items.get_link(s.trim()).ok_or(anyhow!("Invalid item"))?;
        Command::new(config.browser_command_name.as_str())
            .args([link])
            .output()?;
        Ok(())
    } else {
        Err(anyhow!("i3-msg failed"))
    }
}
