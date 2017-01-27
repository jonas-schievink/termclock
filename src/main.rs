extern crate time;
extern crate pancurses;
extern crate libc;

mod draw;
mod font;

static TIME_FORMAT: &'static str = "%H:%M";
static DATE_FORMAT: &'static str = "%d.%m.%Y";

fn main() {
    let main_win = pancurses::initscr();
    pancurses::curs_set(0); // make the cursor disappear
    main_win.erase();

    loop {
        let time = time::now();
        let time_fmt = time::strftime(TIME_FORMAT, &time).unwrap();
        let date_fmt = time::strftime(DATE_FORMAT, &time).unwrap();

        // The double refresh is needed so we correctly respond to a resized terminal
        main_win.refresh();
        main_win.erase();
        draw::draw_text(&time_fmt, &main_win);
        draw::draw_centered(main_win.get_max_y() / 2 + font::FONT_HEIGHT as i32 / 2 + 2,
                            &date_fmt,
                            &main_win);
        main_win.refresh();

        // Use libc's/nix's `sleep` instead of `std::thread::sleep` to be interrupted by signals.
        // "On Unix platforms this function will not return early due to a signal being received or
        // a spurious wakeup."
        unsafe {
            libc::sleep(1);
        }
    }
}
