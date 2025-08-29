use unicode_segmentation::UnicodeSegmentation;

use crate::color::Color;

pub fn rainbowize(text: &str) -> String {
    text.graphemes(true)
        .enumerate()
        .map(|(i, grapheme)| Color::random(i).paint(grapheme))
        .collect::<Vec<_>>()
        .join("")
}
