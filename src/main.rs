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

    /// Show controls
    #[arg(long)]
    controls: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    if args.controls {
        print_controls();
        return Ok(());
    }

    let colors = if let Some(file) = args.colors {
        Notecolors::from_file(file).unwrap_or_default()
    } else {
        Notecolors::default()
    };

    ratatui::run(|term| piano::Piano::new(colors).run(term))
}

fn print_controls() {
    print!(
        "Controls:
        Action      Key(s)

    Device selection:
        up          Up-arrow/k
        down        Down-arrow/j
        confirm     Enter

    Keyboard:
        reset keys  r
        quit        q/Esc
"
    );
}
