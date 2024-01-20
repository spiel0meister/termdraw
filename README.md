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
use crossterm::{
    cursor::SetCursorStyle,
    queue,
    style::Color::*,
    terminal::{Clear, ClearType},
};
use std::{
    io::{stdout, Result, Write},
    thread::sleep,
    time::Duration,
};

use termdraw::shape::{self, *};

fn main() -> Result<()> {
    let mut out = stdout();

    loop {
        queue!(out, Clear(ClearType::All))?;
        queue!(out, SetCursorStyle::SteadyBlock)?;

        draw_background!(out, Black);
        draw_custom_shape!(out, [0, 0, 10, 0, 5, 5], White, true);

        out.flush()?;

        sleep(Duration::from_millis(500));
    }
}
```
