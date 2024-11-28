use libc::{ioctl, winsize, STDERR_FILENO, TIOCGWINSZ};
use std::io::{self, Write};
use std::mem::zeroed;

fn get_terminal_width() -> usize {
    unsafe {
        let mut ws: winsize = zeroed();
        if ioctl(STDERR_FILENO, TIOCGWINSZ, &mut ws) == 0 {
            ws.ws_col as usize
        } else {
            80
        }
    }
}

pub enum PGOutput {
    Stdout,
    Stderr,
}

pub enum PGStyle {
    Percentage,
    Fraction,
}

pub struct ProgressBar {
    max_idx: usize,
    last_idx: usize,
    output: PGOutput,
    style: PGStyle,
}

impl ProgressBar {
    pub fn new(max_idx: usize, style: PGStyle, output: PGOutput) -> Self {
        ProgressBar {
            max_idx,
            last_idx: 0,
            output,
            style,
        }
    }

    pub fn show(&self) {
        let max_width = get_terminal_width();
        let extra_buffer = 5;
        let str_width = match self.style {
            PGStyle::Percentage => 4,
            PGStyle::Fraction => self.max_idx.to_string().len(),
        };
        let max_bar_width = match self.style {
            PGStyle::Percentage => max_width - str_width - extra_buffer,
            PGStyle::Fraction => max_width - (2 * str_width + 1) - extra_buffer,
        };

        let progress_ratio = (self.last_idx - 1) as f64 / self.max_idx as f64;
        let bar_width = (progress_ratio * max_bar_width as f64).round() as usize;

        let current_idx_width = self.last_idx.to_string().len();
        let output_status_str = match self.style {
            PGStyle::Percentage => format!("{}%", (progress_ratio * 100.0).round()),
            PGStyle::Fraction => format!(
                "{}{}/{}",
                " ".repeat(str_width - current_idx_width),
                self.last_idx,
                self.max_idx
            ),
        };
        let output_str = format!(
            "{}{}▎{}",
            "█".repeat(bar_width),
            " ".repeat(max_bar_width - bar_width),
            output_status_str
        );
        if self.last_idx > 1 {
            print!("\x1B[A");
            print!("\x1B[K");
        }
        io::stdout().flush().unwrap();
        match self.output {
            PGOutput::Stdout => {
                println!("{}", output_str);
                io::stdout().flush().unwrap();
            }
            PGOutput::Stderr => {
                eprintln!("{}", output_str);
                io::stderr().flush().unwrap();
            }
        };

        if self.last_idx == self.max_idx {
            println!();
        }
    }

    pub fn update(&mut self) {
        self.last_idx += 1;
    }

    pub fn advance(&mut self, n: usize) {
        self.last_idx += n;
        self.show();
    }
}
