extern crate time;
extern crate pancurses;
extern crate libc;
#[macro_use] extern crate clap;

mod draw;
mod font;

use clap::{App, Arg};


struct Config {
    /// `strftime`-compatible format string for the big (primary) display, usually displaying the
    /// time.
    time_format_string: String,
    /// `strftime`-compatible format string for the small text below the big display (secondary
    /// display), usually displaying the date.
    date_format_string: String,
    /// Delay between clock updates in seconds.
    update_interval: u32,
}

fn parse_args() -> Config {
    let matches = App::new("termclock")
        .author(crate_authors!())
        .version(crate_version!())
        .about("Displays a large clock in your terminal.")
        .arg(Arg::with_name("update_interval")
            .short("s")
            .long("seconds")
            .help("Sets the delay between time updates in seconds")
            .default_value("1")
            .validator(|s| s.parse::<u32>()
                .map(|_| ())
                .map_err(|e| e.to_string())))
        .arg(Arg::with_name("date_fmt")
            .short("d")
            .long("date")
            .help("Format string to use for the date (small text at the bottom)")
            .default_value("%x")
            .validator(|s| time::strftime(&s, &time::now())
                .map(|_| ())
                .map_err(|e| e.to_string())))
        .arg(Arg::with_name("time_fmt")
            .short("t")
            .long("time")
            .help("Format string to use for the time (large text)")
            .default_value("%H:%M")
            .validator(|s| time::strftime(&s, &time::now())
                .map(|_| ())
                .map_err(|e| e.to_string())))
        .get_matches();

    Config {
        update_interval: matches.value_of("update_interval").unwrap().parse().unwrap(),
        date_format_string: matches.value_of("date_fmt").unwrap().to_string(),
        time_format_string: matches.value_of("time_fmt").unwrap().to_string(),
    }
}

fn main() {
    let config = parse_args();

    let main_win = pancurses::initscr();
    pancurses::curs_set(0); // make the cursor disappear
    main_win.erase();

    loop {
        let time = time::now();
        let time_fmt = time::strftime(&config.time_format_string, &time).unwrap();
        let date_fmt = time::strftime(&config.date_format_string, &time).unwrap();

        // The double refresh is needed so we correctly respond to a resized terminal
        main_win.refresh();
        main_win.erase();
        draw::draw_text(&time_fmt, &main_win);
        draw::draw_centered(main_win.get_max_y() / 2 + font::FONT_HEIGHT as i32 / 2 + 2,
                            &date_fmt,
                            &main_win);
        main_win.refresh();

        // Use libc's `sleep` instead of `std::thread::sleep` to be interrupted by signals.
        // "On Unix platforms this function will not return early due to a signal being received or
        // a spurious wakeup."
        unsafe {
            libc::sleep(config.update_interval);
        }
    }
}
