# termdraw

A library, which allows you to draw in the terminal.

## Quick Start

...

## Exapmle

```rust
use std::io::{stdout, Write, Result};
use crossterm::{
    terminal::{Clear, ClearType, size},
    style::Color::*,
    cursor::SetCursorStyle,
    queue
};
use termdraw::shape::*;

fn main() -> Result<()> {
    let mut out = stdout();

    loop {
        queue!(out, Clear(ClearType::All))?;
        queue!(out, SetCursorStyle::SteadyBlock)?;

        draw_background!(out, Black)?;
        draw_rect!(out, 50, 10, 100, 100, White, Black)?;
        out.flush()?;
    }
}
```
