use std::{
    error, fmt, ops,
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

pub trait Day {
    fn day() -> usize;

    fn input() -> &'static str;
}

#[macro_export]
macro_rules! day {
    ($day: expr) => {
        paste::paste! {
            pub struct [<Day $day>];

            impl advent_of_code::Day for [<Day $day>] {
                fn day() -> usize {
                    $day
                }

                fn input() -> &'static str {
                    include_str!(concat!("../input/", stringify!($day), ".txt"))
                }
            }
        }
    };
}

pub trait Solution<'a>: Day {
    type Input: ops::Deref;
    type ParseError: error::Error;

    type P1: fmt::Debug + PartialEq + Send + 'static;
    type P2: fmt::Debug + PartialEq + Send + 'static;

    fn parse(input: &'a str) -> Result<Self::Input, Self::ParseError>;

    fn part1(input: &<Self::Input as ops::Deref>::Target) -> Self::P1;

    fn part2(input: &<Self::Input as ops::Deref>::Target) -> Self::P2;

    fn solve(input: &'a str) -> Result<(Self::P1, Self::P2), Self::ParseError> {
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
}

#[macro_export]
macro_rules! days {
    ($($day: expr => ($ans1: expr, $ans2: expr)),*) => {
        paste::paste! {
            let mut days = Vec::with_capacity(32);
            $(
                days.push([<day $day>]::[<Day $day>]::run([<day $day>]::[<Day $day>]::input(), $ans1, $ans2));
            )*
        }

        let (_, parallel_dur) = advent_of_code::bench(move || {
            for day in days {
                let _handle = day.join();
            }
        });

        println!(
            "\nTotal (parallel): {}ms",
            advent_of_code::format_dur(parallel_dur)
        );
    };
}
