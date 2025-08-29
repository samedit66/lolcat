use std::f32::consts::PI;

pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    pub fn random(seed: usize) -> Self {
        let seed = seed as f32;
        let f = 0.1;

        let red: u8 = ((f * seed).sin() * 127.0 + 128.0) as u8;
        let green: u8 = ((f * seed + 2.0 * PI / 3.0).sin() * 127.0 + 128.0) as u8;
        let blue: u8 = ((f * seed + 4.0 * PI / 3.0).sin() * 127.0 + 128.0) as u8;

        Self { red, blue, green }
    }

    pub fn paint(&self, text: &str) -> String {
        format!(
            "\x1b[38;2;{};{};{}m{}\x1b[0m",
            self.red, self.blue, self.green, text
        )
    }
}
