use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Paragraph, Widget},
};

use crate::{colors::Notecolors, piano::Piano};

#[derive(Debug)]
// The basic keyboard to render
pub struct Keyboard {
    colors: Notecolors,
    pressed_keys: [bool; 88],
}

impl Keyboard {
    pub fn new(colors: Notecolors) -> Self {
        Self {
            colors: colors,
            pressed_keys: [false; 88],
        }
    }

    pub fn press_key(&mut self, key: u16) {
        self.pressed_keys[key as usize] = true;
    }

    pub fn release_key(&mut self, key: u16) {
        self.pressed_keys[key as usize] = false;
    }
}

impl Widget for &Keyboard {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        // considerations:
        //  if the terminal is smaller than 88, we render an error message/ask the user to resize
        // if the terminal is big enough, render each key one-wide
        // if still big enough, render white keys double width, black one?
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

        let pressed_nat = Style::default()
            .bg(self.colors.natural_key)
            .fg(self.colors.natural_key);
        let pressed_acc = Style::default()
            .bg(self.colors.accidental_key)
            .fg(self.colors.accidental_key);

        let accidental_percentage = 0.6;

        for i in 0u16..88 {
            let accidental = Piano::is_accidental(i);
            let pressed = self.pressed_keys[i as usize];

            if !accidental {
                let cell = Rect::new((area.x + i) * width_scale, area.y, width_scale, area.height);
                buf.set_style(cell, if pressed { pressed_nat } else { white });
                // buf.set_string(cell.x, cell.y + area.height - 1, " ", white);
            } else {
                let key_height =
                    (area.height as f64 / (1.0 / accidental_percentage)).max(1.0) as u16;
                let key = Rect::new((area.x + i) * width_scale, area.y, width_scale, key_height);
                buf.set_style(key, if pressed { pressed_acc } else { black });

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
