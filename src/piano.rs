use std::io;

use crossterm::event;
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Paragraph, Widget},
};

#[derive(Debug, Default)]
pub struct Piano {
    keyboard: Keyboard,
    pianoroll: Keyroll,
    exit: bool,
}

impl Piano {
    pub fn run(&mut self, term: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            term.draw(|frame| self.draw(frame))?;

            if event::read()?.is_key_press() {
                self.exit = true;
            }
        }

        Ok(())
    }

    fn is_accidental(key: u16) -> bool {
        match key % 12 {
            1 | 4 | 6 | 9 | 11 => true,
            _ => false,
        }
    }

    fn draw(&self, frame: &mut Frame) {
        // cut the area in smaller pieces, so the piano is at the bottom
        let piano_height = 5;

        let area = Rect::new(
            0,
            frame.area().height - piano_height,
            frame.area().width,
            piano_height,
        );

        frame.render_widget(&self.keyboard, area);
    }
}

#[derive(Debug, Default)]
// The basic keyboard to render
struct Keyboard {}

impl Widget for &Keyboard {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        // considerations:
        //  if the terminal is smaller than 88, we render an error message/ask the user to resize
        // if the terminal is big enough, render each key one-wide
        // if still big enough, render white keys double width, black one
        // maybe more options?
        if area.width < 88 || area.height == 0 {
            let msg = Line::from("Terminal too small!".bold());
            Paragraph::new(msg).centered().render(area, buf);
            return;
        }
        //
        // Paragraph::new(Line::from(format!("Rect: {:?}", area).bold()))
        //     .centered()
        //     .render(area, buf);
        // return;

        let width_scale = if area.width > 88 * 2 { 2 } else { 1 };

        let white = Style::default().bg(Color::White).fg(Color::Black);
        let black = Style::default().bg(Color::Black).fg(Color::White);

        let accidental_percentage = 0.6;

        for i in 0u16..88 {
            let accidental = Piano::is_accidental(i);

            if !accidental {
                let cell = Rect::new((area.x + i) * width_scale, area.y, width_scale, area.height);
                buf.set_style(cell, white);
                // buf.set_string(cell.x, cell.y + area.height - 1, " ", white);
            } else {
                let key_height =
                    (area.height as f64 / (1.0 / accidental_percentage)).max(1.0) as u16;
                let key = Rect::new((area.x + i) * width_scale, area.y, width_scale, key_height);
                buf.set_style(key, black);

                let lower = Rect::new(
                    (area.x + i) * width_scale,
                    area.y + key_height,
                    width_scale,
                    area.height,
                );
                buf.set_style(lower, white);
            }
        }
    }
}

#[derive(Debug)]
// Renders all the currently pressed keys and scrolls them
struct Keyroll {
    keys: [u32; 88],
}

impl Default for Keyroll {
    fn default() -> Self {
        Keyroll { keys: [0; 88] }
    }
}

impl Widget for &Keyroll {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        todo!()
    }
}
