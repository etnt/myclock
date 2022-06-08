use anyhow::{bail, Result};
use crossterm::{cursor::MoveTo, queue, style::Print};
use std::io::{stdout, Stdout, Write};

type Frame = Vec<Vec<&'static str>>;

// These spinner chars was stolen from the `gitui` code.
static SPINNER_CHARS: &[char] = &['⣷', '⣯', '⣟', '⡿', '⢿', '⣻', '⣽', '⣾'];

#[derive(Clone, Copy)]
pub struct Spinner {
    x: usize,
    y: usize,
    pos: usize,
}

impl Spinner {
    pub fn new(x: usize, y: usize, pos: usize) -> Self {
        Self { x, y, pos: pos % 8 }
    }
    pub fn draw(&self) -> Result<()> {
        let mut stdout = stdout();
        queue!(
            stdout,
            MoveTo(self.x as u16, self.y as u16),
            Print(SPINNER_CHARS[self.pos]),
        )
        .unwrap();
        stdout.flush().unwrap();
        Ok(())
    }
    pub fn bump(&mut self) -> Result<()> {
        self.pos = (self.pos + 1) % 8;
        Ok(())
    }
}

pub struct FancyColon {
    c1: Spinner,
    c2: Spinner,
}

impl FancyColon {
    pub fn new(x: usize, y: usize, pos: usize) -> Self {
        Self {
            c1: Spinner::new(x, y + 1, pos),
            c2: Spinner::new(x, y + 3, pos),
        }
    }
    pub fn draw(&mut self) -> Result<()> {
        self.c1.draw()?;
        self.c2.draw()?;
        self.c1.bump()?;
        self.c2.bump()?;
        Ok(())
    }
}

/// A 5x5 box anchored to a starting position.
/// The box will know how to draw a number given
/// to the `draw` funtion.
/// As an optimization, the box will not redraw an
/// identical, already drawn, number.
pub struct Widget5x5 {
    x: usize,
    y: usize,
    cur_num: usize,
}

impl Widget5x5 {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            cur_num: usize::MAX,
        }
    }

    pub fn draw(&mut self, stdout: &mut Stdout, number: usize) -> Result<()> {
        if number == self.cur_num {
            return Ok(());
        }
        draw_empty(stdout, self.x, self.y);
        match number {
            0 => draw_widget(stdout, self.x, self.y, zero_5x5_frame()),
            1 => draw_widget(stdout, self.x, self.y, one_5x5_frame()),
            2 => draw_widget(stdout, self.x, self.y, two_5x5_frame()),
            3 => draw_widget(stdout, self.x, self.y, three_5x5_frame()),
            4 => draw_widget(stdout, self.x, self.y, four_5x5_frame()),
            5 => draw_widget(stdout, self.x, self.y, five_5x5_frame()),
            6 => draw_widget(stdout, self.x, self.y, six_5x5_frame()),
            7 => draw_widget(stdout, self.x, self.y, seven_5x5_frame()),
            8 => draw_widget(stdout, self.x, self.y, eight_5x5_frame()),
            9 => draw_widget(stdout, self.x, self.y, nine_5x5_frame()),
            _ => bail!("wrong widget number given"),
        }
        self.cur_num = number;
        Ok(())
    }
}

/// Draw an empty 5x5 box.
pub fn draw_empty(stdout: &mut Stdout, start_x: usize, start_y: usize) {
    draw_widget(stdout, start_x, start_y, empty_5x5_frame());
}

/// Draw the given box.
pub fn draw_widget(stdout: &mut Stdout, start_x: usize, start_y: usize, frame: Frame) {
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

/// Representing a `colon` character.
//pub fn colon_frame() -> Frame {
//    vec![vec![" "], vec!["O"], vec![" "], vec!["O"], vec![" "]]
//}

/// Representing an empty 5x5 box.
pub fn empty_5x5_frame() -> Frame {
    vec![
        vec![" ", " ", " ", " ", " "],
        vec![" ", " ", " ", " ", " "],
        vec![" ", " ", " ", " ", " "],
        vec![" ", " ", " ", " ", " "],
        vec![" ", " ", " ", " ", " "],
    ]
}

/// Representing the number `1` as a 5x5 box.
pub fn one_5x5_frame() -> Frame {
    vec![
        vec![" ", " ", " ", "X", " "],
        vec![" ", " ", "X", "X", " "],
        vec![" ", " ", " ", "X", " "],
        vec![" ", " ", " ", "X", " "],
        vec![" ", " ", " ", "X", " "],
    ]
}

/// Representing the number `2` as a 5x5 box.
pub fn two_5x5_frame() -> Frame {
    vec![
        vec!["X", "X", "X", "X", " "],
        vec![" ", " ", " ", "X", "X"],
        vec![" ", "X", "X", "X", " "],
        vec!["X", "X", " ", " ", " "],
        vec!["X", "X", "X", "X", "X"],
    ]
}

/// Representing the number `3` as a 5x5 box.
pub fn three_5x5_frame() -> Frame {
    vec![
        vec!["X", "X", "X", "X", " "],
        vec![" ", " ", " ", "X", "X"],
        vec![" ", "X", "X", "X", " "],
        vec![" ", " ", " ", "X", "X"],
        vec!["X", "X", "X", "X", " "],
    ]
}

/// Representing the number `4` as a 5x5 box.
pub fn four_5x5_frame() -> Frame {
    vec![
        vec![" ", "X", " ", " ", "X"],
        vec![" ", "X", " ", " ", "X"],
        vec![" ", "X", "X", "X", "X"],
        vec![" ", " ", " ", " ", "X"],
        vec![" ", " ", " ", " ", "X"],
    ]
}

/// Representing the number `5` as a 5x5 box.
pub fn five_5x5_frame() -> Frame {
    vec![
        vec![" ", "X", "X", "X", "X"],
        vec![" ", "X", " ", " ", " "],
        vec![" ", "X", "X", "X", "X"],
        vec![" ", " ", " ", " ", "X"],
        vec![" ", "X", "X", "X", "X"],
    ]
}

/// Representing the number `6` as a 5x5 box.
pub fn six_5x5_frame() -> Frame {
    vec![
        vec![" ", "X", "X", "X", " "],
        vec!["X", " ", " ", " ", " "],
        vec!["X", "X", "X", "X", " "],
        vec!["X", " ", " ", " ", "X"],
        vec![" ", "X", "X", "X", ""],
    ]
}

/// Representing the number `7` as a 5x5 box.
pub fn seven_5x5_frame() -> Frame {
    vec![
        vec![" ", "X", "X", "X", "X"],
        vec![" ", " ", " ", "X", " "],
        vec![" ", " ", "X", " ", " "],
        vec![" ", " ", "X", " ", " "],
        vec![" ", " ", "X", " ", " "],
    ]
}

/// Representing the number `8` as a 5x5 box.
pub fn eight_5x5_frame() -> Frame {
    vec![
        vec![" ", "X", "X", "X", " "],
        vec!["X", " ", " ", " ", "X"],
        vec![" ", "X", "X", "X", " "],
        vec!["X", " ", " ", " ", "X"],
        vec![" ", "X", "X", "X", " "],
    ]
}

/// Representing the number `9` as a 5x5 box.
pub fn nine_5x5_frame() -> Frame {
    vec![
        vec![" ", "X", "X", "X", " "],
        vec!["X", " ", " ", " ", "X"],
        vec![" ", "X", "X", "X", "X"],
        vec![" ", " ", " ", " ", "X"],
        vec![" ", "X", "X", "X", " "],
    ]
}

/// Representing the number `0` as a 5x5 box.
pub fn zero_5x5_frame() -> Frame {
    vec![
        vec![" ", "X", "X", "X", " "],
        vec!["X", " ", " ", " ", "X"],
        vec!["X", " ", " ", " ", "X"],
        vec!["X", " ", " ", " ", "X"],
        vec![" ", "X", "X", "X", " "],
    ]
}
