use quick_links_rofi::{
    config::get_configuration,
    input::Items,
    link::{launch_link, launch_rofi},
};

fn main() -> anyhow::Result<()> {
    let config = get_configuration()?;
    let items = Items::try_from(config.input_file.as_str())?;
    let chosen = launch_rofi(&items, &config)?;
    match chosen {
        None => Ok(()),
        Some(s) => launch_link(&s, &items, &config),
    }
}
