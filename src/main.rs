use std::{
    io::{stdout, Write, Error, Result, ErrorKind},
    thread::sleep,
    time::Duration
};
use crossterm::{
    QueueableCommand,
    terminal::{Clear, ClearType, size},
    style::Color::*, cursor::{SetCursorStyle, MoveTo}, queue
};

mod shape;
use shape::Drawable;

fn main() -> Result<()> {
    let mut out = stdout();

    loop {
        queue!(out, Clear(ClearType::All))?;
        queue!(out, SetCursorStyle::SteadyBlock)?;

        let (w, h) = size()?;
        shape::draw_background!(out, Black);
        shape::draw_circle!(out, w / 2, h / 2, 1, White, Black);

        queue!(out, MoveTo(w - 1, h - 1))?;

        out.flush()?;

        sleep(Duration::from_millis(500));
    }
}
