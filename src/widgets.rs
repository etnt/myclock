use anyhow::{bail, Result};
use crossterm::{cursor::MoveTo, queue, style::Print};
use std::io::{Stdout, Write};

type Frame = Vec<Vec<&'static str>>;

/// A 5x5 box anchored to a starting position.
/// The box will know how to draw a number given
/// to the `draw` funtion.
/// As an optimization, the box will not redraw an
/// identical, already drawn, number.
pub struct Widget5x5 {
    x: i32,
    y: i32,
    cur_num: i32,
}

impl Widget5x5 {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            cur_num: i32::MAX,
        }
    }

    pub fn draw(&mut self, stdout: &mut Stdout, number: i32) -> Result<()> {
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
pub fn draw_empty(stdout: &mut Stdout, start_x: i32, start_y: i32) {
    draw_widget(stdout, start_x, start_y, empty_5x5_frame());
}

/// Draw the given box.
pub fn draw_widget(stdout: &mut Stdout, start_x: i32, start_y: i32, frame: Frame) {
    for (y, col) in frame.iter().enumerate() {
        for (x, s) in col.iter().enumerate() {
            let (i, j) = rotate((start_x, start_y), (x as i32, y as i32));
            queue!(stdout, MoveTo((i) as u16, (j) as u16), Print(*s),).unwrap();
        }
    }
    stdout.flush().unwrap();
}

struct Pos {
    x: i32,
    y: i32,
}
pub fn rotate(start_pos: (i32, i32), cur_pos: (i32, i32)) -> (i32, i32) {
    let (sx, sy) = start_pos;
    let (cx, cy) = cur_pos;
    let mut pos = Pos::new(cx, cy);
    pos.translate(-sx, -sy).rot90().translate(sx, sy);

    (pos.x, pos.y)
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn translate(&mut self, tx: i32, ty: i32) -> &mut Pos {
        self.x += tx;
        self.y += ty;
        self
    }

    pub fn rot90(&mut self) -> &mut Pos {
        self.multiply(Matrix2x2::matrix90counterclockwise())
    }

    pub fn multiply(&mut self, m: Matrix2x2) -> &mut Pos {
        let x = self.x;
        let y = self.y;
        self.x = m.a * x + m.b * y;
        self.y = m.c * x + m.d * y;
        self
    }
}

struct Matrix2x2 {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
}

impl Matrix2x2 {
    pub fn matrix90counterclockwise() -> Self {
        Self {
            a: 0,
            b: -1,
            c: 1,
            d: 0,
        }
    }
}

/// Representing a `colon` character.
pub fn colon_frame() -> Frame {
    vec![vec![" "], vec!["O"], vec![" "], vec!["O"], vec![" "]]
}

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
