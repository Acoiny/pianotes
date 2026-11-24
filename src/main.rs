use std::io;

mod keyboard;
mod piano;

fn main() -> io::Result<()> {
    ratatui::run(|term| piano::Piano::default().run(term))
}
