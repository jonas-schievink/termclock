extern crate time;
extern crate term;
extern crate term_size;

mod draw;

use std::time::Duration;
use std::thread::sleep;

static TIME_FORMAT: &'static str = "%H:%M";

fn main() {
    let term = term::stdout().unwrap();

    loop {
        let time = time::now();

        let formatted = time::strftime(TIME_FORMAT, &time);

        sleep(Duration::from_secs(1));
    }
}
