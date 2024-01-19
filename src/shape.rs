//! A module that makes it possible to draw shapes.

use crossterm::{
    cursor::MoveTo,
    queue,
    style::{Color, Print, SetBackgroundColor},
    terminal::size,
};
use std::io::{Error, Result, Stdout};

/// A drawable is something, that can be drawn in the terminal.
pub trait Drawable {
    fn draw(&self, stdout: &mut Stdout, stroke_color: Color, fill_color: Color) -> Result<()>;
}

/// A struct that makes it possible to draw a background.
///
/// # Example
///
/// ```
/// let out = stdout();
/// Background.draw(out, Color::Black, Color::Reset);
/// ```
/// You can also use the macro:
/// ```
/// let out = stdout();
/// draw_background!(out, Color::Black);
/// ```
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

/// A struct that makes it possible to draw a point.
///
/// # Example
///
/// ```
/// let out = stdout();
/// Point(0, 0).draw(&mut out, Color::White, Color::Reset);
/// ```
/// You can also use the macro:
/// ```
/// let out = stdout();
/// draw_point!(out, 0, 0, Color::White);
/// ```
pub struct Point(pub u16, pub u16);

impl Drawable for Point {
    fn draw(&self, stdout: &mut Stdout, stroke_color: Color, _fill_color: Color) -> Result<()> {
        queue!(stdout, MoveTo(self.0, self.1))?;
        queue!(stdout, SetBackgroundColor(stroke_color))?;
        queue!(stdout, Print(" "))?;

        Ok(())
    }
}

/// A struct that makes it possible to draw a line.
///
/// # Example
///
/// ```
/// let out = stdout();
/// Line(0, 0, 10, 10).draw(&mut out, Color::White, Color::Reset);
/// ```
/// You can also use the macro:
/// ```
/// let out = stdout();
/// draw_line!(out, 0, 0, 10, 10, Color::White);
/// ```
pub struct Line(pub u16, pub u16, pub u16, pub u16);

impl Drawable for Line {
    fn draw(&self, stdout: &mut Stdout, stroke_color: Color, _fill_color: Color) -> Result<()> {
        queue!(stdout, SetBackgroundColor(stroke_color))?;
        if self.0 == self.2 {
            for y_offset in 0..self.3 {
                queue!(stdout, MoveTo(self.0, self.1 + y_offset))?;
                queue!(stdout, Print(" "))?;
            }
        } else if self.1 == self.3 {
            for x_offset in 0..self.2 {
                queue!(stdout, MoveTo(self.0 + x_offset, self.1))?;
                queue!(stdout, Print(" "))?;
            }
        } else {
            let y_delta = self.3 as i32 - self.1 as i32;
            let x_chunks = (self.2 as i32 - self.0 as i32) / y_delta;
            for y_offset in 0..y_delta + 1 {
                for x_offset in (x_chunks * y_offset)..(x_chunks * y_offset + 1) {
                    queue!(
                        stdout,
                        MoveTo(
                            (self.0 as i32 + x_offset) as u16,
                            (self.1 as i32 + y_offset) as u16
                        )
                    )?;
                    queue!(stdout, Print(" "))?;
                }
            }
        }

        Ok(())
    }
}

/// A struct that makes it possible to draw custom shapes.
///
/// # Example
///
/// ```
/// let out = stdout();
/// let custom_shape = CustomShape(vec![
///     Point(0, 0),
///     Point(10, 0),
///     Point(5, 5)
/// ], true);
/// custom_shape.draw(&mut out, Color::White, Color::Reset);
/// ```
///
/// You can also use the macro:
///
/// ```
/// let out = stdout();
/// draw_custom_shape!(out, [0, 0, 10, 0, 5, 5], Color::White, true);
/// ```
pub struct CustomShape(pub Vec<Point>, pub bool);

impl Drawable for CustomShape {
    fn draw(&self, stdout: &mut Stdout, stroke_color: Color, fill_color: Color) -> Result<()> {
        if self.0.len() < 3 {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "Not enough vertecies!",
            ));
        }

        for i in 0..self.0.len() - 1 {
            let cur = &self.0[i];
            let next = &self.0[i + 1];

            Line(cur.0, cur.1, next.0, next.1).draw(stdout, stroke_color, fill_color)?;
        }

        if self.1 {
            let first = self.0.first().unwrap();
            let last = self.0.last().unwrap();
            Line(first.0, first.1, last.0, last.1).draw(stdout, stroke_color, fill_color)?;
        }

        Ok(())
    }
}

/// A struct that makes it possible to draw a rectangle.
///
/// # Example
///
/// ```
/// let out = stdout();
/// Rect(0, 0, 10, 10).draw(&mut out, Color::Black, Color::Reset)
/// ```
/// You can also use the macro:
/// ```
/// let out = stdout();
/// draw_rect!(out, 0, 0, 10, 10, Color::White, Color::Black);
/// ```
/// If you need to draw a Square, you can also use the `draw_square` macro:
/// ```
/// let out = stdout();
/// draw_square!(out, 0, 0, 10, Color::White, Color::Black);
/// ```
pub struct Rect(pub u16, pub u16, pub u16, pub u16);

impl Drawable for Rect {
    fn draw(&self, stdout: &mut Stdout, stroke_color: Color, fill_color: Color) -> Result<()> {
        for x_offset in 0..self.2 {
            for y_offset in 0..self.3 {
                let x = self.0 + x_offset;
                let y = self.1 + y_offset;
                queue!(stdout, MoveTo(x, y))?;
                if x_offset == 0
                    || x_offset == self.2 - 1
                    || y_offset == 0
                    || y_offset == self.3 - 1
                {
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

/// A struct that makes it possible to draw a circle.
///
/// # Example
///
/// ```
/// let out = stdout();
/// Circle(0, 0, 10).draw(&mut out, Color::Black, Color::Reset)
/// ```
/// You can also use the macro:
/// ```
/// let out = stdout();
/// draw_circle!(out, 0, 0, 10, Color::White, Color::Black);
/// ```
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

/// A macro that makes it possible to draw a background. See `Background`.
#[macro_export]
macro_rules! draw_background {
    ($out:ident, $background_color:expr) => {
        crate::shape::Background.draw(
            &mut $out,
            $background_color,
            crossterm::style::Color::Reset,
        )?;
    };
}

/// A macro that makes it possible to draw a point. See `Point`.
#[macro_export]
macro_rules! draw_point {
    ($out:ident, $x:expr, $y:expr, $point_color:expr) => {
        crate::shape::Point($x, $y).draw(
            &mut $out,
            $point_color,
            crossterm::style::Color::Reset,
        )?;
    };
}

/// A macro that makes it possible to draw custom shapes. See `CustomShape`.
#[macro_export]
macro_rules! draw_custom_shape {
    ($out:ident, [$($x:expr, $y:expr),+], $stroke_color:expr, $close:literal) => {
        {
            let mut points = Vec::new();
            $(
                points.push(Point($x, $y));
            )+;
            CustomShape(points, $close).draw(&mut $out, $stroke_color, crossterm::style::Color::Reset)?;
        }
    };
}

/// A macro that makes it possible to draw a line. See `Line`.
#[macro_export]
macro_rules! draw_line {
    ($out:ident, $x1:expr, $y1:expr, $x2:expr, $y2:expr, $stroke_color:expr) => {
        crate::shape::Line($x1, $y1, $x2, $y2).draw(
            &mut $out,
            $stroke_color,
            crossterm::style::Color::Reset,
        )?;
    };
}

/// A macro that makes it possible to draw a rectangle. See `Rect`.
#[macro_export]
macro_rules! draw_rect {
    ($out:ident, $x:expr, $y:expr, $w:expr, $h:expr, $stroke_color:expr, $fill_color:expr) => {
        crate::shape::Rect($x, $y, $w, $h).draw(&mut $out, $stroke_color, $fill_color)?;
    };
}

/// A macro that makes it possible to draw a square.
#[macro_export]
macro_rules! draw_square {
    ($out:ident, $x:expr, $y:expr, $a:expr, $stroke_color:expr, $fill_color:expr) => {
        crate::shape::Rect($x, $y, $a, $a).draw(&mut $out, $stroke_color, $fill_color)?;
    };
}

/// A macro that makes it possible to draw a circle. See `Circle`.
#[macro_export]
macro_rules! draw_circle {
    ($out:ident, $x:expr, $y:expr, $r:expr, $stroke_color:expr, $fill_color:expr) => {
        crate::shape::Circle($x, $y, $r).draw(&mut $out, $stroke_color, $fill_color)?;
    };
}

pub use draw_background;
pub use draw_circle;
pub use draw_custom_shape;
pub use draw_line;
pub use draw_point;
pub use draw_rect;
pub use draw_square;
