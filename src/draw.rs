//! Contains logic for drawing text in big letters to the terminal.

// Theoretically we could use rusttype to display a *real* font...
// hmmm... we can use a few unicode chars to nicely typeset the font
// and maybe use multiple term colors to get more shades of gray.
// The hardest part is finding fonts, let's leave that to fontconfig
// for good Linux integration.

use term_size;

/// Fixed 5x5 font used to render only numbers for now
static FONT: &'static [&'static [&'static str]] = &[
    [
        "01110",
        "10001",
        "10001",
        "10001",
        "01110",
    ],
    [
        "00001",
        "00001",
        "00001",
        "00001",
        "00001",
    ],
    [
        "11111",
        "00001",
        "11111",
        "10000",
        "11111",
    ],
    [
        "11111",
        "00001",
        "11111",
        "00001",
        "11111",
    ],
    [
        "10001",
        "10001",
        "11111",
        "00001",
        "00001",
    ],
    [
        "11111",
        "10000",
        "11111",
        "00001",
        "11111",
    ],
    [
        "11111",
        "10000",
        "11111",
        "10001",
        "11111",
    ],
    [
        "11111",
        "00010",
        "00100",
        "01000",
        "10000",
    ],
    [
        "11111",
        "10001",
        "11111",
        "10001",
        "11111",
    ],
    [
        "11111",
        "10001",
        "11111",
        "00001",
        "11111",
    ],
    [   // :
        "00000",
        "00100",
        "00000",
        "00100",
        "00000",
    ],
];

pub fn draw_text(text: &str) -> String {
    let (width, height) = term_size::dimensions().unwrap();
    unimplemented!();
}
