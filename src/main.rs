use quick_links_rofi::{
    input::Items,
    link::{launch_link, launch_rofi},
};

fn main() -> anyhow::Result<()> {
    let file = "input.txt";
    let items = Items::try_from(file)?;
    let chosen = launch_rofi(&items)?;
    launch_link(&chosen, &items)?;
    Ok(())
}
