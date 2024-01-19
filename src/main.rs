use crossterm::{
    cursor::SetCursorStyle,
    queue,
    style::Color::*,
    terminal::{size, Clear, ClearType},
};
use std::{
    io::{stdout, Result, Write},
    thread::sleep,
    time::Duration,
};

use termdraw::shape::{self, *};

fn main() -> Result<()> {
    let mut out = stdout();

    loop {
        queue!(out, Clear(ClearType::All))?;
        queue!(out, SetCursorStyle::SteadyBlock)?;

        let (w, h) = size()?;
        draw_background!(out, Black);
        draw_custom_shape!(out, [0, 0, 10, 0, 5, 5], White, true);

        // queue!(out, MoveTo(w - 1, h - 1))?;

        out.flush()?;

        sleep(Duration::from_millis(500));
    }
}
