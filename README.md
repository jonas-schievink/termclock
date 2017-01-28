# Termclock

Termclock is a very simple but beautiful clock for your terminal. By default, it will display the time in big letters as well as the date in normal text below.


## Install

Termclock isn't yet published on [crates.io](https://crates.io), nor is it available in any distributions.

This is the recommended installation method:

* Clone this repository
* Make sure you have [Rust](https://rust-lang.org) installed
* Inside the repository root directory, run `cargo install`

If you just want to try it out without installing, you can instead run `cargo run` to see Termclock in action.

Happy ricing!

## Platform support

The primary target platform is, of course, Linux, but in theory Termclock should work on any platform supported by Rust and ncurses.

Since the ncurses wrapper used by Termclock, [pancurses](https://github.com/ihalila/pancurses), also claims to support Windows, it should be possible to get Termclock running there, too. Yay for cross-platform ricing!
