use std::io::{stdout, Write, Error, Result, ErrorKind};
use crossterm::{
    QueueableCommand,
    terminal::{Clear, ClearType, size},
    style::Color::*
};

mod shape;
use shape::Drawable;

fn main() -> Result<()> {
    let mut out = stdout();

    loop {
        out.queue(Clear(ClearType::All))?;

        let (w, h) = size()?;
        shape::draw_background!(out, Black);
        shape::draw_point!(out, w / 2, h / 2, Black);

        out.flush()?;
    }
}
