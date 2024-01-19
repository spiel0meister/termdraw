# termdraw

A crate, which allows you to draw in the terminal.

## Quick Start

You can find this crate on [crates.io](https://crates.io/crates/termdraw).

You can use cargo:
```console
cargo add termdraw
```
Or include `termdraw = "*"` in the `Cargo.toml` file.

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
