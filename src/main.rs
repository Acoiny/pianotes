use clap::Parser;
use std::io;

use crate::colors::Notecolors;

mod colors;
mod keyboard;
mod keyroll;
mod piano;

#[derive(Parser)]
#[command(about = "Piano visualizer in your terminal")]
struct Args {
    /// Config file to use for colors
    #[arg(short, long)]
    colors: Option<String>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let colors = if let Some(file) = args.colors {
        Notecolors::from_file("colors.toml").unwrap_or_default()
    } else {
        Notecolors::default()
    };

    ratatui::run(|term| piano::Piano::new(colors).run(term))
}
