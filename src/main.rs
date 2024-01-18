use std::{
    io::{stdout, Write, Result},
    thread::sleep,
    time::Duration
};
use crossterm::{
    terminal::{Clear, ClearType, size},
    style::Color::*, cursor::{SetCursorStyle, MoveTo}, queue
};

mod shape;
use shape::*;

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
