
pub const FONT_HEIGHT: usize = 5;
//pub const FONT_WIDTH: usize = 3;

/// Fixed monospace font used to render the time.
/// 
/// Note that this isn't how the font will be rendered. Instead, each string defines a row of
/// "pixels", each of which is displayed according to the logic inside `draw.rs`.
pub static FONT: &'static [[&'static str; FONT_HEIGHT]] = &[
    [
        "###",
        "# #",
        "# #",
        "# #",
        "###",
    ],
    [
        "  #",
        "  #",
        "  #",
        "  #",
        "  #",
    ],
    [
        "###",
        "  #",
        "###",
        "#  ",
        "###",
    ],
    [
        "###",
        "  #",
        "###",
        "  #",
        "###",
    ],
    [
        "# #",
        "# #",
        "###",
        "  #",
        "  #",
    ],
    [
        "###",
        "#  ",
        "###",
        "  #",
        "###",
    ],
    [
        "###",
        "#  ",
        "###",
        "# #",
        "###",
    ],
    [
        "###",
        "  #",
        "  #",
        "  #",
        "  #",
    ],
    [
        "###",
        "# #",
        "###",
        "# #",
        "###",
    ],
    [
        "###",
        "# #",
        "###",
        "  #",
        "###",
    ],
    [   // :
        "   ",
        " # ",
        "   ",
        " # ",
        "   ",
    ],
];
