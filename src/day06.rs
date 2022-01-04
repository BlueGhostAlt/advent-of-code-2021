use std::{num::ParseIntError, str};

advent_of_code::day!(06);

const MAX_TIME: usize = 8;

#[derive(Debug, Clone, Copy)]
pub struct Fish {
    timers: [u64; MAX_TIME + 1],
}

impl Fish {
    const RESET: usize = 6;
    fn add_timer(&mut self, t: usize) -> Result<(), ParseError> {
        let timer = self.timers.get_mut(t).ok_or(ParseError::TimerTooHigh)?;

        *timer += 1;
        Ok(())
    }

    fn next(&mut self) {
        let spawners = self.timers[0];

        self.timers.rotate_left(1);

        self.timers[Self::RESET] += spawners;
    }

    fn next_n(&mut self, n: usize) {
        for _ in 0..n {
            self.next();
        }
    }

    fn count(&self) -> u64 {
        self.timers.iter().sum()
    }
}

impl str::FromStr for Fish {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.trim().split(',').map(|n| n.parse::<usize>()).try_fold(
            Self { timers: [0; 9] },
            |mut fish, t| {
                fish.add_timer(t?)?;

                Ok(fish)
            },
        )
    }
}

impl<'a> advent_of_code::Solution<'a> for Day06 {
    type Input = Box<Fish>;
    type ParseError = ParseError;

    type P1 = u64;
    type P2 = u64;

    fn parse(input: &'a str) -> Result<Self::Input, Self::ParseError> {
        input.parse().map(Box::new)
    }

    fn part1(input: &Fish) -> Self::P1 {
        let mut fish = *input;
        fish.next_n(80);

        fish.count()
    }

    fn part2(input: &Fish) -> Self::P2 {
        let mut fish = *input;
        fish.next_n(256);

        fish.count()
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    TimerTooHigh,
    NotAnInt(ParseIntError),
}

impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> Self {
        Self::NotAnInt(err)
    }
}

use std::{error, fmt};

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimerTooHigh => write!(f, "timer too high, must be between 0 and {}", MAX_TIME),
            Self::NotAnInt(parse_int_err) => write!(f, "{}", parse_int_err),
        }
    }
}

impl error::Error for ParseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::NotAnInt(parse_int_err) => Some(parse_int_err),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use advent_of_code::Solution;

    const INPUT: &str = "3,4,3,1,2";

    #[test]
    fn test() {
        assert_eq!(super::Day06::solve(INPUT), Ok((5934, 26984457539)));
    }
}
