use std::{io::{Stdout, Result}, f32::consts::PI};
use crossterm::{
    QueueableCommand,
    cursor::MoveTo,
    style::{Print, SetBackgroundColor, Color},
    terminal::size
};

fn to_rad(degrees: u16) -> u16 {
    (degrees * PI as u16) / 180 as u16
}

pub trait Drawable {
    fn draw(&self, stdout: &mut Stdout, stroke_color: Color, fill_color: Color) -> Result<()>;
}

pub struct Background;

impl Drawable for Background {
    fn draw(&self, stdout: &mut Stdout, stroke_color: Color, _fill_color: Color) -> Result<()> {
        let (w, h) = size()?;

        stdout.queue(SetBackgroundColor(stroke_color))?;
        for x in 0..w - 1 {
            for y in 0..h - 1 {
                stdout.queue(MoveTo(x, y))?;
                stdout.queue(Print(" "))?;
            }
        }

        Ok(())
    }
}

pub struct Point(pub u16, pub u16);

impl Drawable for Point {
    fn draw(&self, stdout: &mut Stdout, stroke_color: Color, _fill_color: Color) -> Result<()> {
        stdout.queue(MoveTo(self.0, self.1))?;
        stdout.queue(SetBackgroundColor(stroke_color))?;
        stdout.queue(Print(" "))?;

        Ok(())
    }
}

pub struct Circle(pub u16, pub u16, pub u16);

impl Drawable for Circle {
    fn draw(&self, stdout: &mut Stdout, stroke_color: Color, fill_color: Color) -> Result<()> {
        let (w, h) = size()?;
        stdout.queue(SetBackgroundColor(stroke_color))?;
        for a in 0..359 {
            let x_offset = (self.2 as f32 * (to_rad(a) as f32).cos()).round() as u16;
            let y_offset = (self.2 as f32 * (to_rad(a) as f32).sin()).round() as u16;
            let x = self.0 + x_offset;
            let y = self.1 + y_offset;
            if x >= w || y >= h {
                continue;
            }

            stdout.queue(MoveTo(x, y))?;
            stdout.queue(Print(" "))?;
        }

        stdout.queue(SetBackgroundColor(fill_color))?;
        for r in 0..self.2 - 1 {
            for a in 0..359 {
                let x_offset = (r as f32 * (to_rad(a) as f32).cos()).round() as u16;
                let y_offset = (r as f32 * (to_rad(a) as f32).sin()).round() as u16;
                let x = self.0 + x_offset;
                let y = self.1 + y_offset;
                if x >= w || y >= h {
                    continue;
                }

                stdout.queue(MoveTo(x, y))?;
                stdout.queue(Print(" "))?;
            }
        }

        Ok(())
    }
}

#[macro_export]
macro_rules! draw_background {
    ($out:ident, $background_color:expr) => {
        shape::Background.draw(&mut $out, $background_color, crossterm::style::Color::Reset)?;
    };
}

#[macro_export]
macro_rules! draw_point {
    ($out:ident, $x:expr, $y:expr, $point_color:expr) => {
        shape::Point($x, $y).draw(&mut $out, $point_color, crossterm::style::Color::Reset)?;
    };
}

pub use draw_background;
pub use draw_point;
