use ratatui::prelude::Color;

#[derive(Debug, Clone, Copy)]
pub struct Notecolors {
    // colors for the scroll
    pub natural: Color,
    pub accidental: Color,
    // colors for pressed keys
    pub natural_key: Color,
    pub accidental_key: Color,
}

impl Default for Notecolors {
    fn default() -> Self {
        // let nat = Color::Rgb(195, 53, 252);
        let nat = Color::from_u32(0xaf4bed);
        // let acc = Color::Rgb(114, 9, 155);
        // let acc = Color::from_u32(0x19CB00);
        let acc = Color::from_u32(0x5C1489);

        Self {
            natural: nat,
            accidental: acc,
            natural_key: nat,
            accidental_key: acc,
        }
    }
}
