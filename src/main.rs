use std::time::Duration;

use indicatif::{
    HumanDuration, ParallelProgressIterator, ProgressBar, ProgressStyle,
};
use prison::Boxes;
use rand::RngCore;
use rayon::prelude::*;

pub fn main() {
    const N: usize = 100;
    const SAMPLES: u64 = 1_000_000_000;

    let progress =
        ProgressBar::new(SAMPLES)
            .with_style(
                ProgressStyle::default_bar()
                    .template(
                        "[{elapsed}] {bar:80.green/blue} {pos:>7}/{len:7} {my_eta}",
                    )
                    .unwrap()
                    .progress_chars("#>-")
                    .with_key("my_eta", |s| match (s.pos(), s.len()) {
                        (0, _) => "-".to_string(),
                        (pos, _) => format!(
                            "{:#}",
                            HumanDuration(Duration::from_secs_f64(
                                s.elapsed().as_secs_f64() * (SAMPLES - pos) as f64 / pos as f64
                            ))
                        ),
                    }),
            );

    let (dumb, smart): (u64, u64) = (0..SAMPLES)
        .into_par_iter()
        .progress_with(progress)
        .map(|_| {
            let mut board = Boxes::<N>::new_random(rand::thread_rng().next_u64());

            (board.solve_dumb_no_list(), board.solve_smart_cycle_detect())
        })
        .map(|(a, b)| (a as u64, b as u64))
        .reduce(
            || (0_u64, 0_u64),
            |(dumb, smart), (dumb_, smart_)| (dumb + dumb_, smart + smart_),
        );

    println!(
        "Avg: {:} (expected: {:})",
        dumb as f64 / SAMPLES as f64,
        1.0 / 2.0_f64.powf(N as f64)
    );
    println!(
        "Avg: {:} (expected: {:})",
        smart as f64 / SAMPLES as f64,
        1.0 - (N / 2 + 1..=N).map(|i| 1.0 / i as f64).sum::<f64>()
    );
}
