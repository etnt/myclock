use anyhow::{bail, Result};
use crossterm::{cursor::MoveTo, queue, style::Print};
use std::io::{Stdout, Write};

type Frame = Vec<Vec<&'static str>>;

pub struct Widget5x5 {
    x: usize,
    y: usize,
    cur_num: usize, // to optimize: do not redraw an identical number
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

pub fn draw_empty(stdout: &mut Stdout, start_x: usize, start_y: usize) {
    draw_widget(stdout, start_x, start_y, empty_5x5_frame());
}

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

pub fn empty_5x5_frame() -> Frame {
    vec![
        vec![" ", " ", " ", " ", " "],
        vec![" ", " ", " ", " ", " "],
        vec![" ", " ", " ", " ", " "],
        vec![" ", " ", " ", " ", " "],
        vec![" ", " ", " ", " ", " "],
    ]
}

pub fn one_5x5_frame() -> Frame {
    vec![
        vec![" ", " ", " ", " ", "X"],
        vec![" ", " ", " ", "X", "X"],
        vec![" ", " ", " ", " ", "X"],
        vec![" ", " ", " ", " ", "X"],
        vec![" ", " ", " ", " ", "X"],
    ]
}

pub fn two_5x5_frame() -> Frame {
    vec![
        vec!["X", "X", "X", "X", " "],
        vec![" ", " ", " ", "X", "X"],
        vec![" ", "X", "X", "X", " "],
        vec!["X", "X", " ", " ", " "],
        vec!["X", "X", "X", "X", "X"],
    ]
}

pub fn three_5x5_frame() -> Frame {
    vec![
        vec!["X", "X", "X", "X", " "],
        vec![" ", " ", " ", "X", "X"],
        vec![" ", "X", "X", "X", " "],
        vec![" ", " ", " ", "X", "X"],
        vec!["X", "X", "X", "X", " "],
    ]
}

pub fn four_5x5_frame() -> Frame {
    vec![
        vec![" ", "X", " ", " ", "X"],
        vec![" ", "X", " ", " ", "X"],
        vec![" ", "X", "X", "X", "X"],
        vec![" ", " ", " ", " ", "X"],
        vec![" ", " ", " ", " ", "X"],
    ]
}

pub fn five_5x5_frame() -> Frame {
    vec![
        vec![" ", "X", "X", "X", "X"],
        vec![" ", "X", " ", " ", " "],
        vec![" ", "X", "X", "X", "X"],
        vec![" ", " ", " ", " ", "X"],
        vec![" ", "X", "X", "X", "X"],
    ]
}

pub fn six_5x5_frame() -> Frame {
    vec![
        vec![" ", "X", "X", "X", " "],
        vec!["X", " ", " ", " ", " "],
        vec!["X", "X", "X", "X", " "],
        vec!["X", " ", " ", " ", "X"],
        vec![" ", "X", "X", "X", ""],
    ]
}

pub fn seven_5x5_frame() -> Frame {
    vec![
        vec![" ", "X", "X", "X", "X"],
        vec![" ", " ", " ", "X", " "],
        vec![" ", " ", "X", " ", " "],
        vec![" ", " ", "X", " ", " "],
        vec![" ", " ", "X", " ", " "],
    ]
}

pub fn eight_5x5_frame() -> Frame {
    vec![
        vec![" ", "X", "X", "X", " "],
        vec!["X", " ", " ", " ", "X"],
        vec![" ", "X", "X", "X", " "],
        vec!["X", " ", " ", " ", "X"],
        vec![" ", "X", "X", "X", " "],
    ]
}

pub fn nine_5x5_frame() -> Frame {
    vec![
        vec![" ", "X", "X", "X", " "],
        vec!["X", " ", " ", " ", "X"],
        vec![" ", "X", "X", "X", "X"],
        vec![" ", " ", " ", " ", "X"],
        vec![" ", "X", "X", "X", " "],
    ]
}

pub fn zero_5x5_frame() -> Frame {
    vec![
        vec![" ", "X", "X", "X", " "],
        vec!["X", " ", " ", " ", "X"],
        vec!["X", " ", " ", " ", "X"],
        vec!["X", " ", " ", " ", "X"],
        vec![" ", "X", "X", "X", " "],
    ]
}
