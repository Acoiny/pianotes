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
        let mut area = frame.area();
        let piano_height = 5;
        area.y = area.height - piano_height;
        area.height = piano_height;
        frame.render_widget(self, area);
    }

    // fn draw(&self, frame: &mut Frame) {
    //     let colors = [Color::White, Color::Black, Color::Blue, Color::Red];
    //     let canvas = Canvas::default()
    //         .x_bounds([0., 88.]) // make size double of what is needed, so rectangle works
    //         .y_bounds([0., 100.])
    //         .marker(Marker::Block)
    //         .paint(|ctx| {
    //             // draw 88 keys
    //             for i in 0..88 {
    //                 let accidental = Piano::is_accidental(i);
    //
    //                 let color = if accidental {
    //                     Color::Black
    //                 } else {
    //                     Color::White
    //                 };
    //
    //                 let offset = if accidental { 1.0 } else { 0.0 };
    //
    //                 ctx.draw(&Line::new(
    //                     (i as f64),
    //                     offset,
    //                     (i as f64),
    //                     10. - offset,
    //                     color,
    //                 ));
    //                 // ctx.draw(&Rectangle::new(
    //                 //     (i as f64) * 2.0,
    //                 //     offset,
    //                 //     1.0,
    //                 //     10.0 - offset,
    //                 //     color,
    //                 // ));
    //
    //                 if accidental {
    //                     ctx.draw(&Line::new((i as f64), 0., (i as f64), offset, Color::White));
    //                     // ctx.draw(&Rectangle::new(
    //                     //     (i as f64) * 2.0,
    //                     //     0.0,
    //                     //     1.0,
    //                     //     offset,
    //                     //     Color::White,
    //                     // ));
    //                 }
    //                 // ctx.draw(&Rectangle::new(
    //                 //     2.0,
    //                 //     10.0,
    //                 //     1.0,
    //                 //     10.0,
    //                 //     if 1 % 2 == 0 {
    //                 //         Color::White
    //                 //     } else {
    //                 //         Color::Black
    //                 //     },
    //                 // ));
    //                 //
    //                 // ctx.draw(&Line::new(
    //                 //     i as f64,
    //                 //     0.0,
    //                 //     i as f64,
    //                 //     10.0,
    //                 //     if i % 2 == 0 {
    //                 //         Color::White
    //                 //     } else {
    //                 //         Color::Black
    //                 //     },
    //                 // ));
    //             }
    //         });
    //
    //     frame.render_widget(canvas, frame.area());
    // }
}

impl Widget for &Piano {
    fn render(self, area: Rect, buf: &mut Buffer) {
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
