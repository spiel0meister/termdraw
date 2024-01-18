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
use shape::Drawable;

fn main() -> Result<()> {
    let mut out = stdout();

    loop {
        queue!(out, Clear(ClearType::All))?;
        queue!(out, SetCursorStyle::SteadyBlock)?;

        let (w, h) = size()?;
        shape::draw_background!(out, Black);
        shape::draw_square!(out, w/2, h/2, 5, White, Black);

        queue!(out, MoveTo(w - 1, h - 1))?;

        out.flush()?;

        sleep(Duration::from_millis(500));
    }
}
