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
use widgets::Widget5x5;

fn main() {
    let mut stdout = io::stdout();
    setup_terminal(&mut stdout).unwrap();

    let delay = time::Duration::from_millis(1000);

    let mut hw1 = Widget5x5::new(1, 1);
    let mut hw2 = Widget5x5::new(8, 1);
    let mut mw1 = Widget5x5::new(15, 1);
    let mut mw2 = Widget5x5::new(22, 1);
    let mut sw1 = Widget5x5::new(29, 1);
    let mut sw2 = Widget5x5::new(36, 1);

    loop {
        if is_event_available().unwrap() {
            break;
        }
        let (h1, h2, m1, m2, s1, s2) = current_time();
        hw1.draw(&mut stdout, h1).unwrap();
        hw2.draw(&mut stdout, h2).unwrap();
        mw1.draw(&mut stdout, m1).unwrap();
        mw2.draw(&mut stdout, m2).unwrap();
        sw1.draw(&mut stdout, s1).unwrap();
        sw2.draw(&mut stdout, s2).unwrap();
        thread::sleep(delay);
    }

    // 'top: for i in 0..=1 {
    //     for j in 0..=2 {
    //         if is_event_available().unwrap() {
    //             break 'top;
    //         }
    //         w2.draw(&mut stdout, j).unwrap();
    //         thread::sleep(delay);
    //     }
    //     w1.draw(&mut stdout, i).unwrap();
    //     thread::sleep(delay);
    // }

    teardown_terminal(&mut stdout).unwrap();

    let (h1, h2, m1, m2, s1, s2) = current_time();
    println!("Hour: {}{}:{}{}:{}{}", h1, h2, m1, m2, s1, s2);

    exit(0);
}

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
