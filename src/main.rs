use anyhow::Result;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    queue,
    style::Print,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io::{self, Stdout, Write},
    thread, time,
};

type Frame = Vec<Vec<&'static str>>;

fn main() {
    setup_terminal().unwrap();

    let mut stdout = io::stdout();
    let delay = time::Duration::from_millis(1000);

    draw(&mut stdout, 1, 1, one_5x5_frame());
    thread::sleep(delay);
    draw(&mut stdout, 1, 1, empty_5x5_frame());

    draw(&mut stdout, 1, 1, two_5x5_frame());
    thread::sleep(delay);
    draw(&mut stdout, 1, 1, empty_5x5_frame());

    draw(&mut stdout, 1, 1, three_5x5_frame());
    thread::sleep(delay);
    draw(&mut stdout, 1, 1, empty_5x5_frame());

    draw(&mut stdout, 1, 1, four_5x5_frame());
    thread::sleep(delay);
    draw(&mut stdout, 1, 1, empty_5x5_frame());

    draw(&mut stdout, 1, 1, five_5x5_frame());
    thread::sleep(delay);
    draw(&mut stdout, 1, 1, empty_5x5_frame());

    draw(&mut stdout, 1, 1, six_5x5_frame());
    thread::sleep(delay);
    draw(&mut stdout, 1, 1, empty_5x5_frame());

    draw(&mut stdout, 1, 1, seven_5x5_frame());
    thread::sleep(delay);
    draw(&mut stdout, 1, 1, empty_5x5_frame());

    draw(&mut stdout, 1, 1, eight_5x5_frame());
    thread::sleep(delay);
    draw(&mut stdout, 1, 1, empty_5x5_frame());

    draw(&mut stdout, 1, 1, nine_5x5_frame());
    thread::sleep(delay);
    draw(&mut stdout, 1, 1, empty_5x5_frame());

    draw(&mut stdout, 1, 1, zero_5x5_frame());
    thread::sleep(delay);
    draw(&mut stdout, 1, 1, empty_5x5_frame());

    teardown_terminal().unwrap();
}

fn draw(stdout: &mut Stdout, start_x: usize, start_y: usize, frame: Frame) {
    for (y, col) in frame.iter().enumerate() {
        for (x, s) in col.iter().enumerate() {
            queue!(
                stdout,
                MoveTo((start_x + x) as u16, (start_y + y) as u16),
                Print(*s),
            )
            .unwrap();
        }
    }
    stdout.flush().unwrap();
}

fn empty_5x5_frame() -> Frame {
    vec![
        vec![" ", " ", " ", " ", " "],
        vec![" ", " ", " ", " ", " "],
        vec![" ", " ", " ", " ", " "],
        vec![" ", " ", " ", " ", " "],
        vec![" ", " ", " ", " ", " "],
    ]
}

fn one_5x5_frame() -> Frame {
    vec![
        vec![" ", " ", " ", " ", "X"],
        vec![" ", " ", " ", "X", "X"],
        vec![" ", " ", " ", " ", "X"],
        vec![" ", " ", " ", " ", "X"],
        vec![" ", " ", " ", " ", "X"],
    ]
}

fn two_5x5_frame() -> Frame {
    vec![
        vec!["X", "X", "X", "X", " "],
        vec![" ", " ", " ", "X", "X"],
        vec![" ", "X", "X", "X", " "],
        vec!["X", "X", " ", " ", " "],
        vec!["X", "X", "X", "X", "X"],
    ]
}

fn three_5x5_frame() -> Frame {
    vec![
        vec!["X", "X", "X", "X", " "],
        vec![" ", " ", " ", "X", "X"],
        vec![" ", "X", "X", "X", " "],
        vec![" ", " ", " ", "X", "X"],
        vec!["X", "X", "X", "X", " "],
    ]
}

fn four_5x5_frame() -> Frame {
    vec![
        vec![" ", "X", " ", " ", "X"],
        vec![" ", "X", " ", " ", "X"],
        vec![" ", "X", "X", "X", "X"],
        vec![" ", " ", " ", " ", "X"],
        vec![" ", " ", " ", " ", "X"],
    ]
}

fn five_5x5_frame() -> Frame {
    vec![
        vec![" ", "X", "X", "X", "X"],
        vec![" ", "X", " ", " ", " "],
        vec![" ", "X", "X", "X", "X"],
        vec![" ", " ", " ", " ", "X"],
        vec![" ", "X", "X", "X", "X"],
    ]
}

fn six_5x5_frame() -> Frame {
    vec![
        vec![" ", "X", "X", "X", " "],
        vec!["X", " ", " ", " ", " "],
        vec!["X", "X", "X", "X", " "],
        vec!["X", " ", " ", " ", "X"],
        vec![" ", "X", "X", "X", ""],
    ]
}

fn seven_5x5_frame() -> Frame {
    vec![
        vec![" ", "X", "X", "X", "X"],
        vec![" ", " ", " ", "X", " "],
        vec![" ", " ", "X", " ", " "],
        vec![" ", " ", "X", " ", " "],
        vec![" ", " ", "X", " ", " "],
    ]
}

fn eight_5x5_frame() -> Frame {
    vec![
        vec![" ", "X", "X", "X", " "],
        vec!["X", " ", " ", " ", "X"],
        vec![" ", "X", "X", "X", " "],
        vec!["X", " ", " ", " ", "X"],
        vec![" ", "X", "X", "X", " "],
    ]
}

fn nine_5x5_frame() -> Frame {
    vec![
        vec![" ", "X", "X", "X", " "],
        vec!["X", " ", " ", " ", "X"],
        vec![" ", "X", "X", "X", "X"],
        vec![" ", " ", " ", " ", "X"],
        vec![" ", "X", "X", "X", " "],
    ]
}

fn zero_5x5_frame() -> Frame {
    vec![
        vec![" ", "X", "X", "X", " "],
        vec!["X", " ", " ", " ", "X"],
        vec!["X", " ", " ", " ", "X"],
        vec!["X", " ", " ", " ", "X"],
        vec![" ", "X", "X", "X", " "],
    ]
}

/// Enter 'raw mode', 'AlternateScreen' and 'Hide' the ursor.
fn setup_terminal() -> Result<()> {
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    crossterm::execute!(stdout, EnterAlternateScreen, Hide)?;
    Ok(())
}

/// Show the Cursor, leave the 'AlternateScreen' and the 'raw mode'.
fn teardown_terminal() -> Result<()> {
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, Show, LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
