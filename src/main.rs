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
use widgets::Widget5x5;

fn main() {
    let mut stdout = io::stdout();
    setup_terminal(&mut stdout).unwrap();

    let delay = time::Duration::from_millis(1000);

    let mut w = Widget5x5::new(1, 1);
    for i in 0..=9 {
        w.draw(&mut stdout, i).unwrap();
        thread::sleep(delay);
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
