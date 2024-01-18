use std::io::{Stdout, Result};
use crossterm::{
    cursor::MoveTo,
    style::{Print, SetBackgroundColor, Color},
    terminal::size,
    queue
};

pub trait Drawable {
    fn draw(&self, stdout: &mut Stdout, stroke_color: Color, fill_color: Color) -> Result<()>;
}

pub struct Background;

impl Drawable for Background {
    fn draw(&self, stdout: &mut Stdout, stroke_color: Color, _fill_color: Color) -> Result<()> {
        let (w, h) = size()?;

        queue!(stdout, SetBackgroundColor(stroke_color))?;
        for x in 0..w - 1 {
            for y in 0..h - 1 {
                queue!(stdout, MoveTo(x, y))?;
                queue!(stdout, Print(" "))?;
            }
        }

        Ok(())
    }
}

pub struct Point(pub u16, pub u16);

impl Drawable for Point {
    fn draw(&self, stdout: &mut Stdout, stroke_color: Color, _fill_color: Color) -> Result<()> {
        queue!(stdout, MoveTo(self.0, self.1))?;
        queue!(stdout, SetBackgroundColor(stroke_color))?;
        queue!(stdout, Print(" "))?;

        Ok(())
    }
}

pub struct Rect(pub u16, pub u16, pub u16, pub u16);

impl Drawable for Rect {
    fn draw(&self, stdout: &mut Stdout, stroke_color: Color, fill_color: Color) -> Result<()> {
        for x_offset in 0..self.2 {
            for y_offset in 0..self.3 {
                let x = self.0 + x_offset;
                let y = self.1 + y_offset;
                queue!(stdout, MoveTo(x, y))?;
                if x_offset == 0 || x_offset == self.2 - 1 || y_offset == 0 || y_offset == self.3 - 1 {
                    queue!(stdout, SetBackgroundColor(stroke_color))?;
                } else {
                    queue!(stdout, SetBackgroundColor(fill_color))?;
                }
                queue!(stdout, Print(" "))?;
            }
        }

        Ok(())
    }
}

pub struct Circle(pub u16, pub u16, pub u16);

impl Drawable for Circle {
    fn draw(&self, stdout: &mut Stdout, stroke_color: Color, fill_color: Color) -> Result<()> {
        let r = self.2 as i32;
        let (w, h) = size()?;
        for x_offset in -r..r + 1 {
            for y_offset in -(r - x_offset.abs())..(r - x_offset.abs() + 1) {
                let x = (self.0 as i32 + x_offset) as u16;
                let y = (self.1 as i32 + y_offset) as u16;
                if x >= w || y >= h {
                    continue;
                }

                queue!(stdout, MoveTo(x, y))?;
                if (x_offset + y_offset).abs() == r || (x_offset - y_offset).abs() == r {
                    queue!(stdout, SetBackgroundColor(stroke_color))?;
                } else {
                    queue!(stdout, SetBackgroundColor(fill_color))?;
                }
                queue!(stdout, Print(" "))?;
            }
        }

        Ok(())
    }
}

#[macro_export]
macro_rules! draw_background {
    ($out:ident, $background_color:expr) => {
        crate::shape::Background.draw(&mut $out, $background_color, crossterm::style::Color::Reset)?;
    };
}

#[macro_export]
macro_rules! draw_point {
    ($out:ident, $x:expr, $y:expr, $point_color:expr) => {
        crate::shape::Point($x, $y).draw(&mut $out, $point_color, crossterm::style::Color::Reset)?;
    };
}

#[macro_export]
macro_rules! draw_rect {
    ($out:ident, $x:expr, $y:expr, $w:expr, $h:expr, $stroke_color:expr, $fill_color:expr) => {
        crate::shape::Rect($x, $y, $w, $h).draw(&mut $out, $stroke_color, $fill_color)?;
    };
}

#[macro_export]
macro_rules! draw_square {
    ($out:ident, $x:expr, $y:expr, $a:expr, $stroke_color:expr, $fill_color:expr) => {
        crate::shape::Rect($x, $y, $a, $a).draw(&mut $out, $stroke_color, $fill_color)?;
    };
}

#[macro_export]
macro_rules! draw_circle {
    ($out:ident, $x:expr, $y:expr, $r:expr, $stroke_color:expr, $fill_color:expr) => {
        crate::shape::Circle($x, $y, $r).draw(&mut $out, $stroke_color, $fill_color)?;
    };
}

pub use draw_background;
pub use draw_point;
pub use draw_circle;
pub use draw_rect;
pub use draw_square;
