use std::io;

use crate::colors::Notecolors;

mod colors;
mod keyboard;
mod keyroll;
mod piano;

fn main() -> io::Result<()> {
    let colors = Notecolors::from_file("colors.toml").unwrap_or_default();
    ratatui::run(|term| piano::Piano::new(colors).run(term))
}
