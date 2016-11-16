extern crate time;
extern crate term;
extern crate term_size;
extern crate pancurses;

mod draw;
mod font;

use std::time::Duration;
use std::thread::sleep;

static TIME_FORMAT: &'static str = "%H:%M";
static DATE_FORMAT: &'static str = "%d.%m.%Y";

fn main() {
    let main_win = pancurses::initscr();
    pancurses::curs_set(0); // make the cursor disappear
    main_win.erase();

    let term = term::stdout().unwrap();

    loop {
        let time = time::now();
        let time_fmt = time::strftime(TIME_FORMAT, &time).unwrap();
        let date_fmt = time::strftime(DATE_FORMAT, &time).unwrap();

        main_win.erase();
        let lines = draw::draw_text(&time_fmt, &main_win);
        main_win.refresh();

        sleep(Duration::from_secs(1));
    }
}
