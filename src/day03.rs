use std::str;

advent_of_code::day!(03);

#[derive(Debug)]
pub struct Line {
    inner: Vec<bool>,
}

impl Line {
    fn width(&self) -> usize {
        self.inner.len()
    }

    fn invert(&mut self) {
        self.inner.iter_mut().for_each(|x| *x = !*x);
    }

    fn to_decimal(&self) -> u32 {
        self.inner
            .iter()
            .rev()
            .enumerate()
            .map(|(idx, bit)| match bit {
                true => 2u32.pow(idx as u32),
                false => 0,
            })
            .sum()
    }
}

impl str::FromStr for Line {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner = s
            .bytes()
            .map(|c| match c {
                b'0' => Ok(false),
                b'1' => Ok(true),
                _ => Err(ParseError::UnknownChar(c)),
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { inner })
    }
}

impl<'a> advent_of_code::Solution<'a> for Day03 {
    type Input = Vec<Line>;
    type ParseError = ParseError;

    type P1 = Option<u32>;
    type P2 = Option<u32>;

    fn parse(input: &'a str) -> Result<Self::Input, Self::ParseError> {
        input.lines().map(str::parse).collect()
    }

    fn part1(input: &[Line]) -> Self::P1 {
        let width = input.first()?.width();
        let half = input.len() / 2;

        let mut gamma_rate = Line {
            inner: (0..width)
                .map(|i| input.iter().filter(|d| d.inner[i]).count() > half)
                .collect(),
        };

        let gamma = gamma_rate.to_decimal();
        gamma_rate.invert();
        let epsilon = gamma_rate.to_decimal();

        Some(gamma * epsilon)
    }

    fn part2(input: &[Line]) -> Self::P2 {
        #[derive(Debug, Clone, Copy)]
        enum Rating {
            O2,
            CO2,
        }

        fn filter_records(diagnostics: &mut Vec<&Line>, rating: Rating, bit_pos: usize) {
            let half = diagnostics.len();
            let bits = diagnostics.iter().filter(|d| d.inner[bit_pos]).count();

            let bit = match rating {
                Rating::O2 => bits * 2 >= half,
                Rating::CO2 => bits * 2 < half,
            };

            diagnostics.retain(|l| l.inner[bit_pos] == bit);
        }

        fn get_rating(diagnostics: &[Line], rating: Rating) -> Option<u32> {
            let width = diagnostics.first()?.width();
            let mut diagnostics = diagnostics.iter().collect();

            (0..width).find_map(|i| {
                filter_records(&mut diagnostics, rating, i);

                match diagnostics.first() {
                    Some(l) if diagnostics.len() == 1 => Some(l.to_decimal()),
                    _ => None,
                }
            })
        }

        let o2 = get_rating(input, Rating::O2)?;
        let co2 = get_rating(input, Rating::CO2)?;

        Some(o2 * co2)
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnknownChar(u8),
}

use std::{error, fmt};

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownChar(char) => write!(f, "unknown char \"{}\"", *char as char),
        }
    }
}

impl error::Error for ParseError {}

#[cfg(test)]
mod tests {
    use advent_of_code::Solution;

    const INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn test() {
        assert_eq!(super::Day03::solve(INPUT), Ok((Some(198), Some(230))));
    }
}
