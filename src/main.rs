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

use termdraw::shape::*;

fn main() -> Result<()> {
    let mut out = stdout();

    loop {
        queue!(out, Clear(ClearType::All))?;
        queue!(out, SetCursorStyle::SteadyBlock)?;

        let (w, h) = size()?;
        let custom = CustomShape(vec![Point(0, 0), Point(10, 0), Point(5, 5)]);
        custom.draw(&mut out, White, Reset)?;

        // queue!(out, MoveTo(w - 1, h - 1))?;

        out.flush()?;

        sleep(Duration::from_millis(500));
    }
}
