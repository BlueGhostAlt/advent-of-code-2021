use std::{
    error::Error,
    fmt::{Debug, Display},
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

const MILLISECOND: Duration = Duration::from_millis(1);

pub fn format_dur(dur: Duration) -> String {
    if dur.as_nanos() > MILLISECOND.as_nanos() {
        format!(
            "{:.2}",
            dur.as_nanos() as f64 / MILLISECOND.as_nanos() as f64
        )
    } else {
        format!(
            "{:.3}",
            dur.as_nanos() as f64 / MILLISECOND.as_nanos() as f64
        )
    }
}

pub fn bench<F, R>(fun: F) -> (R, Duration)
where
    F: FnOnce() -> R,
{
    let now = Instant::now();
    let res = fun();

    (res, now.elapsed())
}

pub trait Solution {
    type Input;
    type ParseError: Error;

    type P1: Debug + Display + PartialEq + Send + 'static;
    type P2: Debug + Display + PartialEq + Send + 'static;

    fn parse(input: &str) -> Result<Self::Input, Self::ParseError>;

    fn part1(input: &Self::Input) -> Self::P1;

    fn part2(input: &Self::Input) -> Self::P2;

    fn solve(input: &str) -> Result<(Self::P1, Self::P2), Self::ParseError> {
        let input = Self::parse(input)?;

        let p1 = Self::part1(&input);
        let p2 = Self::part2(&input);

        Ok((p1, p2))
    }

    fn run(input: &'static str, ans1: Self::P1, ans2: Self::P2) -> JoinHandle<()> {
        thread::spawn(move || {
            let input = Self::parse(input).unwrap();

            let (((p1, p1_dur), (p2, p2_dur)), total_dur) = bench(|| {
                let (p1, p1_dur) = bench(|| Self::part1(&input));
                let (p2, p2_dur) = bench(|| Self::part2(&input));

                ((p1, p1_dur), (p2, p2_dur))
            });

            assert_eq!(p1, ans1);
            assert_eq!(p2, ans2);

            println!(
                "Day {} ({}ms):\n    Part 1: {:?} ({}ms)\n    Part 2: {:?} ({}ms)",
                Self::day(),
                format_dur(total_dur),
                p1,
                format_dur(p1_dur),
                p2,
                format_dur(p2_dur)
            );
        })
    }

    fn day() -> usize;
}
