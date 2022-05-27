mod widgets;

use anyhow::Result;
use crossterm::{
    cursor::{Hide, Show},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io::{self, Stdout},
    thread, time,
};
use widgets::{draw, draw_empty};

fn main() {
    let mut stdout = io::stdout();
    setup_terminal(&mut stdout).unwrap();

    let delay = time::Duration::from_millis(1000);

    for i in 0..=9 {
        draw(&mut stdout, 1, 1, i).unwrap();
        thread::sleep(delay);
        draw_empty(&mut stdout, 1, i);
    }

    teardown_terminal(&mut stdout).unwrap();
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
