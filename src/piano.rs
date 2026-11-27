use std::error::Error;
use std::io::{self, ErrorKind};
use std::time::Duration;

use midir::{Ignore, MidiInput, MidiInputPort};

use crossterm::event::{self, KeyCode, KeyModifiers};
use ratatui::prelude::Color;
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph, Widget};
use ratatui::{DefaultTerminal, Frame, layout::Rect};

use crate::colors::Notecolors;
use crate::keyboard::Keyboard;
use crate::keyroll::Keyroll;

#[derive(Debug)]
pub struct Piano {
    keyboard: Keyboard,
    pianoroll: Keyroll,
    exit: bool,
}

// height of the piano in terminal lines
const PIANO_HEIGHT: u16 = 5;

impl Piano {
    pub fn new(colors: Notecolors) -> Self {
        Self {
            keyboard: Keyboard::new(colors),
            pianoroll: Keyroll::new(colors),
            exit: false,
        }
    }

    pub fn run(&mut self, term: &mut DefaultTerminal) -> io::Result<()> {
        let mut height = 0;

        let res = self.setup_midi(term);

        if let Err(err) = res {
            return Err(io::Error::new(ErrorKind::Other, format!("{}", err)));
        }

        let (midi, port) = res.unwrap();

        let (tx, rx) = std::sync::mpsc::channel();

        let _connection = midi
            .connect(
                &port,
                "pianote-midi-input",
                move |_, message, _| {
                    tx.send(message.to_vec()).unwrap(); // TODO: don't unwrap? idk, crash should be fine here
                },
                (),
            )
            .unwrap();

        // main loop
        while !self.exit {
            term.draw(|frame| {
                height = frame.area().height - PIANO_HEIGHT; // TODO: remove, this is for debugging

                self.draw(frame);
            })?;

            // check if there is an event there
            if let Ok(r) = event::poll(Duration::from_secs(0))
                && r
            {
                // if so, read it
                let ev = event::read()?;

                // if keypress, handle it
                if ev.is_key_press() {
                    let ev = ev.as_key_event().unwrap();
                    match ev.code {
                        // allow quitting with Ctrl-C
                        KeyCode::Char('c') => {
                            if ev.modifiers.contains(KeyModifiers::CONTROL) {
                                self.exit = true;
                            }
                        }
                        // if the key is q, also quit
                        KeyCode::Char('q') => self.exit = true,
                        // r to release all keys. Useful when a midi device has been disconnected
                        KeyCode::Char('r') => {
                            (0..88).for_each(|key| self.release_key(key));
                        }
                        _ => {}
                    };
                }
            }

            // sleep for 33 milliseconds ~30 fps
            std::thread::sleep(Duration::from_millis(33));
            self.pianoroll.update(height as i32);

            while let Ok(msg) = rx.try_recv() {
                // check message type
                // we just assume that no messages are malformed, which should
                // be a reasonable approach. midir is responsible for that :P
                match (msg[0] & 0xf0) >> 4 {
                    // note off
                    0b1000 => {
                        let note = msg[1] - 21; // note must be on piano

                        self.release_key(note as u16);
                    }
                    // note on
                    0b1001 => {
                        let note = msg[1] - 21; // note must be on piano
                        let vel = msg[2];

                        if vel == 0 {
                            self.release_key(note as u16);
                        } else {
                            self.press_key(note as u16);
                        }
                    }
                    // ignore all other events for now
                    _ => {}
                }
            }
        }

        Ok(())
    }

    fn release_key(&mut self, key: u16) {
        self.pianoroll.release_key(key as u16);
        self.keyboard.release_key(key as u16);
    }

    fn press_key(&mut self, key: u16) {
        self.pianoroll.press_key(key as u16);
        self.keyboard.press_key(key as u16);
    }

    // Set up the midi device and prompt the user for a midi port, if multiple are visible
    fn setup_midi(
        &self,
        term: &mut DefaultTerminal,
    ) -> Result<(MidiInput, MidiInputPort), Box<dyn Error>> {
        let mut midi_in = MidiInput::new("pianotes")?;
        midi_in.ignore(Ignore::None);

        let ports = midi_in.ports();

        let in_port = match ports.len() {
            0 => return Err("No midi device found!".into()),
            1 => &ports[0],
            _ => {
                let mut selected: i32 = 0;

                loop {
                    term.draw(|frame| {
                        let mut lines = vec![Line::from("Select a device:")];

                        for (i, p) in ports.iter().enumerate() {
                            lines.push(
                                Line::from(format!("{}: {}", i, midi_in.port_name(p).unwrap()))
                                    // set different color, if this is currently selected
                                    .style(Style::default().fg(if i == selected as usize {
                                        Color::Cyan
                                    } else {
                                        Color::White
                                    })),
                            );
                        }

                        Paragraph::new(lines)
                            .block(
                                Block::default()
                                    .borders(Borders::ALL)
                                    .title("Select device")
                                    .border_type(BorderType::Rounded),
                            )
                            .render(frame.area(), frame.buffer_mut());
                    })?;

                    // wait for key press
                    if let Ok(ev) = event::read() {
                        if let Some(key) = ev.as_key_event() {
                            match key.code {
                                // break out of loop and select current
                                KeyCode::Enter | KeyCode::Char('q') | KeyCode::Esc => break,
                                KeyCode::Up | KeyCode::Char('k') => {
                                    selected = (selected - 1).max(0)
                                }
                                KeyCode::Down | KeyCode::Char('j') => {
                                    selected = (selected + 1).min(ports.len() as i32 - 1)
                                }
                                _ => {}
                            };
                        }
                    } else {
                        return Err("Error picking device".into());
                    }
                }

                // selected paragraph
                &ports[selected as usize]
            }
        };

        Ok((midi_in, in_port.clone()))
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
