# pg-indicator-rs

This is simple library for creating progress indicators in terminal.

This is written in Rust.

## How to install

```sh
cargo install --git https://github.com/SNKK62/pg-indicator-rs#main
```

## Usage

This is simple example.

```rust
use pg_indicator::{PGOutput, PGStyle, ProgressBar};
use std::{thread, time};

fn main() {
    let max_i = 100;
    let max_j = 100;

    // Create progress bar
    let mut pb = ProgressBar::new(max_i * max_j, PGStyle::Fraction , PGOutput::Stdout);

    for _ in 0..max_i {
        for _ in 0..max_j {
            let handred_millis = time::Duration::from_micros(100);
            thread::sleep(handred_millis);
            pb.update(); // update progress bar
        }
    }
}
```

This is output.

```
████████████████████████████████                                              ▎ 4160/10000
```

### Options

### PGStyle

| Name | Description |
| ---- | ----------- |
| `Fraction` | Show fraction of progress. |
| `Percentage` | Show percentage of progress. |

### PGOutput

| Name | Description |
| ---- | ----------- |
| `Stdout` | Output to stdout. |
| `Stderr` | Output to stderr. |


