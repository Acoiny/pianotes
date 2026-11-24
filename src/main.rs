use std::io;

mod piano;

fn main() -> io::Result<()> {
    ratatui::run(|term| piano::Piano::default().run(term))
}
