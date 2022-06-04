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
use std::{
    io::{self, Stdout},
    process::exit,
    thread,
    time::{self, Duration},
};
use widgets::{colon_frame, draw_widget, Widget5x5};

/// The main loop is responsible for setting up and tearing down
/// the terminal. It will create the needed widgets and enter a
/// loop that will, for each second, extract the current time and
/// then invoke all the number widgets with the corresponding
/// number in order to have them drawn on the terminal.
fn main() {
    let mut stdout = io::stdout();
    setup_terminal(&mut stdout).unwrap();

    let delay = time::Duration::from_millis(1000);

    let positions = [(1, 1), (8, 1), (18, 1), (25, 1), (36, 1), (43, 1)];

    //let mut hw1 = Widget5x5::new(1, 1);
    let mut hw2 = Widget5x5::new(8, 1);
    //let mut mw1 = Widget5x5::new(18, 1);
    //let mut mw2 = Widget5x5::new(25, 1);
    //let mut sw1 = Widget5x5::new(36, 1);
    //let mut sw2 = Widget5x5::new(43, 1);

    //draw_widget(&mut stdout, 15, 1, colon_frame());
    //draw_widget(&mut stdout, 33, 1, colon_frame());
    loop {
        if is_event_available().unwrap() {
            break;
        }
        let (h1, h2, m1, m2, s1, s2) = current_time();
        //hw1.draw(&mut stdout, h1).unwrap();
        hw2.draw(&mut stdout, h2).unwrap();
        //mw1.draw(&mut stdout, m1).unwrap();
        //mw2.draw(&mut stdout, m2).unwrap();
        //sw1.draw(&mut stdout, s1).unwrap();
        //sw2.draw(&mut stdout, s2).unwrap();
        thread::sleep(delay);
    }

    teardown_terminal(&mut stdout).unwrap();
    exit(0);
}

/// Return the current time, split up in its components as integers.
fn current_time() -> (i32, i32, i32, i32, i32, i32) {
    let local = Local::now();
    let hours = local.hour();
    let minutes = local.minute();
    let seconds = local.second();
    (
        (hours / 10) as i32,
        (hours % 10) as i32,
        (minutes / 10) as i32,
        (minutes % 10) as i32,
        (seconds / 10) as i32,
        (seconds % 10) as i32,
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
