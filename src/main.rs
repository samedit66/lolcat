use std::f32::consts::PI;
use std::io;

use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let lines: Vec<_> = io::stdin().lines().map_while(Result::ok).collect();
    let text = lines.join("\n");
    print!("{}", rainbowize(&text));
}

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

fn rainbowize(text: &str) -> String {
    let mut graphemes: Vec<String> = vec![];

    for (i, grapheme) in text.graphemes(true).enumerate() {
        let c = random_color(i);
        graphemes.push(colorize(grapheme, c));
    }

    graphemes.join("")
}

fn colorize(grapheme: &str, color: Color) -> String {
    format!(
        "\x1b[38;2;{};{};{}m{}\x1b[0m",
        color.red, color.blue, color.green, grapheme
    )
}

fn random_color(seed: usize) -> Color {
    let seed = seed as f32;
    let f = 0.1;

    let red: u8 = ((f * seed).sin() * 127.0 + 128.0) as u8;
    let green: u8 = ((f * seed + 2.0 * PI / 3.0).sin() * 127.0 + 128.0) as u8;
    let blue: u8 = ((f * seed + 4.0 * PI / 3.0).sin() * 127.0 + 128.0) as u8;

    Color { red, blue, green }
}
