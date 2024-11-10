use termion::terminal_size;
use termion::color;

pub struct Terminal {
    pub width: u16,
    pub height: u16,
}

impl Terminal {
    pub fn new() -> Self {
        let (w, h) = terminal_size().unwrap();
        Self {
            width: w,
            height: h,
        }
    }

    pub fn center_pos(&self) -> (u16, u16) {
        (self.width / 2, self.height / 2)
    }
}

#[derive(Clone)]
pub struct Cell {
    pub char: char,
    pub color: color::Rgb,
}