/// # GDC - Great Digital Clock
///
/// This is just a fun little exercise to learn Rust and to
/// playaround with some Terminal graphics.
///
/// A Great Digital Clock is displayed as ASCII graphics;
/// ticking away every second.
///
mod widgets;

use anyhow::Result;
use chrono::{Local, Timelike};
use crossterm::{
    cursor::{Hide, Show},
    event::poll,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use rusty_time::prelude::Timer;
use std::{
    io::{self, Stdout},
    process::exit,
    thread,
    time::{Duration, Instant},
};
use widgets::{FancyColon, Widget5x5};

/// The main loop is responsible for setting up and tearing down
/// the terminal. It will create the needed widgets and enter a
/// loop that will, for each second, extract the current time and
/// then invoke all the number widgets with the corresponding
/// number in order to have them drawn on the terminal.
fn main() {
    let mut stdout = io::stdout();
    setup_terminal(&mut stdout).unwrap();

    let mut clock_delay = Timer::from_millis(1000);
    let mut instant: Instant = Instant::now();

    let mut hw1 = Widget5x5::new(1, 1);
    let mut hw2 = Widget5x5::new(8, 1);
    let mut mw1 = Widget5x5::new(18, 1);
    let mut mw2 = Widget5x5::new(25, 1);
    let mut sw1 = Widget5x5::new(36, 1);
    let mut sw2 = Widget5x5::new(43, 1);

    //draw_widget(&mut stdout, 15, 1, colon_frame());
    //draw_widget(&mut stdout, 33, 1, colon_frame());
    let mut colon1 = FancyColon::new(15, 1, 0);
    let mut colon2 = FancyColon::new(33, 1, 0);

    let mut force = true;
    loop {
        if is_event_available().unwrap() {
            break;
        }

        // We want to do stuff inside a second,
        // hence this delta time handling.
        let delta: Duration = instant.elapsed();
        instant = Instant::now();
        clock_delay.update(delta);

        if clock_delay.ready | force {
            force = false; // force draw the very first time...
            clock_delay.reset();
            let (h1, h2, m1, m2, s1, s2) = current_time();
            hw1.draw(&mut stdout, h1).unwrap();
            hw2.draw(&mut stdout, h2).unwrap();
            mw1.draw(&mut stdout, m1).unwrap();
            mw2.draw(&mut stdout, m2).unwrap();
            sw1.draw(&mut stdout, s1).unwrap();
            sw2.draw(&mut stdout, s2).unwrap();
        }
        colon1.draw().unwrap();
        colon2.draw().unwrap();
        thread::sleep(Duration::from_millis(100));
    }

    teardown_terminal(&mut stdout).unwrap();
    exit(0);
}

/// Return the current time, split up in its components as integers.
fn current_time() -> (usize, usize, usize, usize, usize, usize) {
    let local = Local::now();
    let hours = local.hour();
    let minutes = local.minute();
    let seconds = local.second();
    (
        (hours / 10) as usize,
        (hours % 10) as usize,
        (minutes / 10) as usize,
        (minutes % 10) as usize,
        (seconds / 10) as usize,
        (seconds % 10) as usize,
    )
}

/// Check for any kind of terminal events.
fn is_event_available() -> crossterm::Result<bool> {
    // Zero duration says that the `poll` function must return immediately
    // with an `Event` availability information
    poll(Duration::from_secs(0))
}

/// Enter 'raw mode', 'AlternateScreen' and 'Hide' the ursor.
fn setup_terminal(stdout: &mut Stdout) -> Result<()> {
    terminal::enable_raw_mode()?;
    crossterm::execute!(stdout, EnterAlternateScreen, Hide)?;
    Ok(())
}

/// Show the Cursor, leave the 'AlternateScreen' and the 'raw mode'.
fn teardown_terminal(stdout: &mut Stdout) -> Result<()> {
    crossterm::execute!(stdout, Show, LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
