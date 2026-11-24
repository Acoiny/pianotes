use std::collections::VecDeque;

use ratatui::{buffer::Buffer, layout::Rect, style::Style, widgets::Widget};

use crate::piano::{COLOR_ACCIDENTAL, COLOR_NATURAL, Piano};

#[derive(Debug, Default, Clone, Copy)]
struct Keypress {
    pressed_y: i32,
    released_y: i32,
    released: bool,
}

#[derive(Debug)]
// Renders all the currently pressed keys and scrolls them
pub struct Keyroll {
    keys: [VecDeque<Keypress>; 88],
    height: i32,
}

impl Keyroll {
    pub fn update(&mut self, height: i32) {
        self.height = height;
        // update all key presses
        for key in &mut self.keys {
            for press in key {
                Self::update_single_keypress(press);
            }
        }

        // remove elements that are out of bounds
        // for key in &mut self.keys {
        //     key.retain(|press| {
        //         // Return false when this press is out of bounds.
        //         true
        //     });
        // }

        // for removal we only look at the last element
        for key in &mut self.keys {
            // remove it, if it is released and the released index is out of bounds
            if let Some(last) = key.back()
                && last.released
                && last.released_y < 0
            {
                key.pop_back();
            }
        }
    }

    pub fn press_key(&mut self, key: u16) {
        // DON'T add press, if the last one has NOT been released yet
        if let Some(press) = self.keys[key as usize].front()
            && !press.released
        {
            return;
        }

        // add new key
        self.keys[key as usize].push_front(Keypress {
            pressed_y: self.height,
            released_y: self.height,
            released: false,
        });
    }

    pub fn release_key(&mut self, key: u16) {
        if let Some(press) = self.keys[key as usize].front_mut() {
            press.released = true;
        }
    }

    fn update_single_keypress(key: &mut Keypress) {
        key.pressed_y = (key.pressed_y - 1).max(0);

        if key.released {
            key.released_y -= 1;
        }
    }
}

impl Default for Keyroll {
    fn default() -> Self {
        Keyroll {
            // fill with empty queues
            keys: std::array::from_fn(|_| VecDeque::new()),
            height: 0,
        }
    }
}

impl Widget for &Keyroll {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        if area.width < 88 || area.height == 0 {
            return;
        }
        //
        // Paragraph::new(Line::from(format!("Rect: {:?}", area).bold()))
        //     .centered()
        //     .render(area, buf);
        // return;

        let width_scale = if area.width > 88 * 2 { 2 } else { 1 };

        let col_accidental = Style::default().bg(COLOR_ACCIDENTAL).fg(COLOR_ACCIDENTAL);
        let col_natural = Style::default().bg(COLOR_NATURAL).fg(COLOR_NATURAL);

        for i in 0u16..88 {
            let accidental = Piano::is_accidental(i);

            let col = if accidental {
                col_accidental
            } else {
                col_natural
            };

            for press in &self.keys[i as usize] {
                let height = if press.released {
                    (press.released_y - press.pressed_y) as u16
                } else {
                    area.height - press.pressed_y as u16
                };

                let rect = Rect::new(i * width_scale, press.pressed_y as u16, width_scale, height);
                // eprintln!("Press: {:?}", rect);

                buf.set_style(rect, col);
            }
        }
    }
}
