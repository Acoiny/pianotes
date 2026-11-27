use ratatui::prelude::Color;
use serde::Deserialize;
use std::{
    fs,
    io::{Error, ErrorKind, Result},
    path::Path,
};

#[derive(Debug, Clone, Copy)]
pub struct Notecolors {
    // colors for the scroll
    pub natural: Color,
    pub accidental: Color,
    // colors for pressed keys
    pub natural_key: Color,
    pub accidental_key: Color,
}

#[derive(Deserialize, Debug)]
struct Tomlcolors {
    keys: Option<Colortuple>,
    roll: Option<Colortuple>,
}

#[derive(Deserialize, Debug)]
struct Colortuple {
    natural: Option<u32>,
    accidental: Option<u32>,
}

const DEFAULT_NATURAL: u32 = 0xaf4bed;
const DEFAULT_ACCIDENTAL: u32 = 0x5C1489;

impl Notecolors {
    pub fn from_file<R: AsRef<Path>>(filename: R) -> Result<Self> {
        let cont = fs::read_to_string(filename)?;

        let parsed: Tomlcolors =
            toml::from_str(&cont).map_err(|err| Error::new(ErrorKind::Other, err))?;

        println!("{:?}", parsed);

        let roll_colors = parsed.roll.unwrap_or(Colortuple {
            natural: None,
            accidental: None,
        });
        let keys_colors = parsed.keys.unwrap_or(Colortuple {
            natural: None,
            accidental: None,
        });

        Ok(Notecolors {
            accidental: Color::from_u32(roll_colors.accidental.unwrap_or(DEFAULT_ACCIDENTAL)),
            natural: Color::from_u32(roll_colors.natural.unwrap_or(DEFAULT_NATURAL)),

            accidental_key: Color::from_u32(keys_colors.accidental.unwrap_or(DEFAULT_ACCIDENTAL)),
            natural_key: Color::from_u32(keys_colors.natural.unwrap_or(DEFAULT_NATURAL)),
        })
    }
}

impl Default for Notecolors {
    fn default() -> Self {
        Self {
            natural: Color::from_u32(DEFAULT_NATURAL),
            accidental: Color::from_u32(DEFAULT_ACCIDENTAL),
            natural_key: Color::from_u32(DEFAULT_NATURAL),
            accidental_key: Color::from_u32(DEFAULT_ACCIDENTAL),
        }
    }
}
