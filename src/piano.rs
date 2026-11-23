use std::io;

use crossterm::event;
use ratatui::{
    DefaultTerminal, Frame,
    style::Color,
    symbols::Marker,
    widgets::canvas::{Canvas, FilledLine, Line, Rectangle},
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

    fn is_accidental(key: i32) -> bool {
        match key % 12 {
            1 | 2 | 4 | 6 | 9 | 11 => true,
            _ => false,
        }
    }

    fn draw(&self, frame: &mut Frame) {
        let colors = [Color::White, Color::Black, Color::Blue, Color::Red];
        let canvas = Canvas::default()
            .x_bounds([0., 2.0 * 88.]) // make size double of what is needed, so rectangle works
            .y_bounds([0., 100.])
            .marker(Marker::Block)
            .paint(|ctx| {
                // draw 88 keys
                for i in 0..88 {
                    let accidental = Piano::is_accidental(i);

                    let color = if accidental {
                        Color::Black
                    } else {
                        Color::White
                    };

                    let offset = if accidental { 1.0 } else { 0.0 };

                    ctx.draw(&Rectangle::new(
                        (i as f64) * 2.0,
                        offset,
                        1.0,
                        10.0 - offset,
                        color,
                    ));

                    if accidental {
                        ctx.draw(&Rectangle::new(
                            (i as f64) * 2.0,
                            0.0,
                            1.0,
                            offset,
                            Color::White,
                        ));
                    }
                    // ctx.draw(&Rectangle::new(
                    //     2.0,
                    //     10.0,
                    //     1.0,
                    //     10.0,
                    //     if 1 % 2 == 0 {
                    //         Color::White
                    //     } else {
                    //         Color::Black
                    //     },
                    // ));
                    //
                    // ctx.draw(&Line::new(
                    //     i as f64,
                    //     0.0,
                    //     i as f64,
                    //     10.0,
                    //     if i % 2 == 0 {
                    //         Color::White
                    //     } else {
                    //         Color::Black
                    //     },
                    // ));
                }
            });

        frame.render_widget(canvas, frame.area());
    }
}
