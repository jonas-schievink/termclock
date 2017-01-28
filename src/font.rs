use std::collections::HashMap;

pub const FONT_HEIGHT: usize = 5;
pub const FONT_WIDTH: usize = 3;

/// Fixed monospace font used to render the time.
/// 
/// Note that this isn't how the font will be rendered. Instead, each string defines a row of
/// "pixels", each of which is displayed according to the logic inside `draw.rs`.
static RAW_FONT: &'static [(char, [&'static str; FONT_HEIGHT])] = &[
    ('0', [
        "###",
        "# #",
        "# #",
        "# #",
        "###",
    ]),
    ('1', [
        "  #",
        "  #",
        "  #",
        "  #",
        "  #",
    ]),
    ('2', [
        "###",
        "  #",
        "###",
        "#  ",
        "###",
    ]),
    ('3', [
        "###",
        "  #",
        "###",
        "  #",
        "###",
    ]),
    ('4', [
        "# #",
        "# #",
        "###",
        "  #",
        "  #",
    ]),
    ('5', [
        "###",
        "#  ",
        "###",
        "  #",
        "###",
    ]),
    ('6', [
        "###",
        "#  ",
        "###",
        "# #",
        "###",
    ]),
    ('7', [
        "###",
        "  #",
        "  #",
        "  #",
        "  #",
    ]),
    ('8', [
        "###",
        "# #",
        "###",
        "# #",
        "###",
    ]),
    ('9', [
        "###",
        "# #",
        "###",
        "  #",
        "###",
    ]),
    (':', [
        "   ",
        " # ",
        "   ",
        " # ",
        "   ",
    ]),
];

pub type Font = HashMap<char, [bool; FONT_HEIGHT * FONT_WIDTH]>;

fn make_font() -> Font {
    let mut map = HashMap::new();

    for &(ch, lines) in RAW_FONT {
        assert_eq!(lines.len(), FONT_HEIGHT);

        let mut pixels = [false; FONT_HEIGHT * FONT_WIDTH];
        for (y, line) in lines.iter().enumerate() {
            assert_eq!(line.len(), FONT_WIDTH);

            for (x, ch) in line.chars().enumerate() {
                pixels[FONT_WIDTH * y + x] = match ch {
                    ' ' => false,
                    '#' => true,
                    invalid => panic!("invalid char in font definition: {}", invalid),
                };
            }
        }

        map.insert(ch, pixels);
    }

    map
}

lazy_static! {
    pub static ref FONT: Font = make_font();
}
