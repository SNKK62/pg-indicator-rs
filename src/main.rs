use pg_indicator::{PGOutput, PGStyle, ProgressBar};
use std::{thread, time};

fn main() {
    let max_i = 100;
    let max_j = 100;

    let mut pb = ProgressBar::new(max_i * max_j, PGStyle::Fraction, PGOutput::Stdout);

    for _ in 0..max_i {
        for _ in 0..max_j {
            let handred_millis = time::Duration::from_micros(100);
            thread::sleep(handred_millis);
            pb.update();
        }
    }
}
