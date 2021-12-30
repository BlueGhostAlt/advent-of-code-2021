use std::{
    error, fmt, ops,
    sync::mpsc::channel,
    thread,
    time::{Duration, Instant},
};

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

#[derive(Debug)]
pub struct DayBench<P1, P2>
where
    P1: fmt::Debug + PartialEq + Send + 'static,
    P2: fmt::Debug + PartialEq + Send + 'static,
{
    pub part1: (P1, Duration),
    pub part2: (P2, Duration),
    pub total: Duration,
}

#[derive(Debug)]
pub enum RunError {
    Unexpected(Box<dyn fmt::Debug + Send>, Box<dyn fmt::Debug + Send>),
}

impl fmt::Display for RunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unexpected(ex, ac) => {
                write!(f, "expected answer {:?} but instead got {:?}", ex, ac)
            }
        }
    }
}

impl error::Error for RunError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
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

    fn run(
        input: &'static str,
        ans1: Self::P1,
        ans2: Self::P2,
    ) -> Result<DayBench<Self::P1, Self::P2>, RunError> {
        let (tx, rx) = channel();

        thread::spawn(move || {
            let input = Self::parse(input).unwrap();

            let (((p1, p1_dur), (p2, p2_dur)), total_dur) = bench(|| {
                let (p1, p1_dur) = bench(|| Self::part1(&input));
                let (p2, p2_dur) = bench(|| Self::part2(&input));

                ((p1, p1_dur), (p2, p2_dur))
            });

            let res = if p1 != ans1 {
                Err(RunError::Unexpected(Box::new(ans1), Box::new(p1)))
            } else if p2 != ans2 {
                Err(RunError::Unexpected(Box::new(ans1), Box::new(p1)))
            } else {
                Ok(DayBench {
                    part1: (p1, p1_dur),
                    part2: (p2, p2_dur),
                    total: total_dur,
                })
            };

            tx.send(res).unwrap();
        });

        rx.recv().unwrap()
    }
}

#[macro_export]
macro_rules! days {
    ($($day: expr => ($ans1: expr, $ans2: expr)),+) => {
        paste::paste! {
            let now = std::time::Instant::now();

            $(
                let [<day_ $day>] = [<day $day>]::[<Day $day>]::run([<day $day>]::[<Day $day>]::input(), $ans1, $ans2).unwrap();
            )+

            let total = now.elapsed();

            $(
                println!(
                    "Day {}({:?}):\n    Part 1({:?}): {:?}\n    Part 2({:?}): {:?}",
                    $day, [<day_ $day>].total, [<day_ $day>].part1.1, [<day_ $day>].part1.0, [<day_ $day>].part2.1, [<day_ $day>].part2.0
                );
            )+
            println!("\nTotal: {:?}", total);
        }
    };
}
