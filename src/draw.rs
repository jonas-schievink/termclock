//! Contains logic for drawing text in big letters to the terminal.

// Theoretically we could use rusttype to display a *real* font...
// hmmm... we can use a few unicode chars to nicely typeset the font
// and maybe use multiple term colors to get more shades of gray.
// The hardest part is finding fonts, let's leave that to fontconfig
// for good Linux integration.

use font::*;

use pancurses::Window;

const CH_FILL: char = '█';
const CH_EMPTY: char = ' ';

pub fn draw_text(text: &str, win: &Window) {
    let mut lines = vec![String::new(); FONT_HEIGHT];

    for (num, ch) in text.chars().enumerate() {
        let index = match ch {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            ':' => 10,
            _ => unimplemented!(),
        };

        let glyph = &FONT[index];
        for (glyph_line, buf_line) in glyph.iter().zip(lines.iter_mut()) {
            if num != 0 {
                // Single pixel space between chars
                buf_line.push(CH_EMPTY);
            }

            for c in glyph_line.chars() {
                let pixel = match c {
                    ' ' => CH_EMPTY,
                    '#' => CH_FILL,
                    _ => panic!("Font contains invalid character"),
                };
                buf_line.push(pixel);
                buf_line.push(pixel);
            }
        }
    }

    let start_y = win.get_max_y() / 2 - lines.len() as i32 / 2;

    for (i, line) in lines.iter().enumerate() {
        draw_centered(start_y + i as i32, line, win);
    }
}

pub fn draw_centered(row: i32, text: &str, win: &Window) {
    let startx = win.get_max_x() / 2 - text.chars().count() as i32 / 2;
    win.mvaddstr(row, startx, text);
}
