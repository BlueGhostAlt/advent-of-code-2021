use std::{collections::HashMap, num::ParseIntError, ops, str};

use advent_of_code::day;

day!(05);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

type Step = Point;

impl ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn is_diagonal(&self) -> bool {
        self.start.x != self.end.x && self.start.y != self.end.y
    }

    fn points(&self) -> Points {
        Points::new(*self)
    }
}

impl str::FromStr for Line {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s
            .split_once("->")
            .ok_or(ParseError::MissingPointsSeparator)?;

        let (x, y) = start
            .trim()
            .split_once(',')
            .ok_or(ParseError::MissingCoordsSeparator)?;
        let start = Point {
            x: x.parse()?,
            y: y.parse()?,
        };

        let (x, y) = end
            .trim()
            .split_once(',')
            .ok_or(ParseError::MissingCoordsSeparator)?;
        let end = Point {
            x: x.parse()?,
            y: y.parse()?,
        };

        Ok(Self { start, end })
    }
}

#[derive(Debug)]
struct Points {
    line: Line,
    step: Step,
}

impl Points {
    fn new(mut line: Line) -> Self {
        let step = Step {
            x: (line.end.x - line.start.x).signum(),
            y: (line.end.y - line.start.y).signum(),
        };
        line.start -= step;

        Self { line, step }
    }
}

impl Iterator for Points {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.line.start == self.line.end {
            None
        } else {
            self.line.start.x += self.step.x;
            self.line.start.y += self.step.y;

            Some(self.line.start)
        }
    }
}

fn count_overlaps<'a, I>(iter: I) -> usize
where
    I: Iterator<Item = &'a Line>,
{
    iter.flat_map(Line::points)
        .fold(HashMap::new(), |mut acc, p| {
            *acc.entry(p).or_insert(0) += 1;
            acc
        })
        .iter()
        .filter(|(_, &f)| f > 1)
        .count()
}

impl<'a> advent_of_code::Solution<'a> for Day05 {
    type Input = Vec<Line>;
    type ParseError = ParseError;

    type P1 = usize;
    type P2 = usize;

    fn parse(input: &'a str) -> Result<Self::Input, Self::ParseError> {
        input.lines().map(str::parse).collect()
    }

    fn part1(input: &[Line]) -> Self::P1 {
        count_overlaps(input.iter().filter(|line| !line.is_diagonal()))
    }

    fn part2(input: &[Line]) -> Self::P2 {
        count_overlaps(input.iter())
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    MissingPointsSeparator,
    MissingCoordsSeparator,
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
            Self::MissingPointsSeparator => write!(f, "missing points separator"),
            Self::MissingCoordsSeparator => write!(f, "missing coords separator"),
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

    const INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test() {
        assert_eq!(super::Day05::solve(INPUT), Ok((5, 12)));
    }
}
