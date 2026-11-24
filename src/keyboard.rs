use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Paragraph, Widget},
};

use crate::piano::Piano;

#[derive(Debug, Default)]
// The basic keyboard to render
pub struct Keyboard {}

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
