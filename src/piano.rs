use std::io;

use crossterm::event::{self, KeyCode};
use ratatui::{DefaultTerminal, Frame, layout::Rect, style::Color};

use crate::keyboard::Keyboard;
use crate::keyroll::Keyroll;

#[derive(Debug, Default)]
pub struct Piano {
    keyboard: Keyboard,
    pianoroll: Keyroll,
    exit: bool,
}

pub const COLOR_NATURAL: ratatui::prelude::Color = Color::Blue;
pub const COLOR_ACCIDENTAL: ratatui::prelude::Color = Color::Cyan;

// height of the piano in terminal lines
const PIANO_HEIGHT: u16 = 5;

impl Piano {
    pub fn run(&mut self, term: &mut DefaultTerminal) -> io::Result<()> {
        let mut height = 0;
        while !self.exit {
            term.draw(|frame| {
                height = frame.area().height - PIANO_HEIGHT; // TODO: remove, this is for debugging

                self.draw(frame);
            })?;

            let ev = event::read()?;

            if ev.is_key_press() {
                match ev.as_key_event().unwrap().code {
                    // if the key is q, quit
                    KeyCode::Char('q') => self.exit = true,
                    KeyCode::Char('p') => {
                        self.pianoroll.press_key(0);
                        self.pianoroll.press_key(1);
                    }
                    KeyCode::Char('r') => {
                        self.pianoroll.release_key(0);
                        self.pianoroll.release_key(1);
                    }
                    // otherwise update a frame
                    _ => self.pianoroll.update(height as i32),
                };
            }
        }

        Ok(())
    }

    pub fn is_accidental(key: u16) -> bool {
        match key % 12 {
            1 | 4 | 6 | 9 | 11 => true,
            _ => false,
        }
    }

    fn draw(&self, frame: &mut Frame) {
        // cut the area in smaller pieces, so the piano is at the bottom

        let keys_area = Rect::new(
            0,
            frame.area().height - PIANO_HEIGHT,
            frame.area().width,
            PIANO_HEIGHT,
        );

        let roll_area = Rect::new(0, 0, frame.area().width, frame.area().height - PIANO_HEIGHT);

        frame.render_widget(&self.keyboard, keys_area);
        frame.render_widget(&self.pianoroll, roll_area);
    }
}
