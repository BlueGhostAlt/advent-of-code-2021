use std::num::ParseIntError;

use advent_of_code::day;

day!(02);

#[derive(Debug)]
pub enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl<'a> advent_of_code::Solution<'a> for Day02 {
    type Input = Vec<Command>;
    type ParseError = ParseError<'a>;

    type P1 = u32;
    type P2 = u32;

    fn parse(input: &'a str) -> Result<Self::Input, Self::ParseError> {
        input
            .lines()
            .map(|l| {
                let (command, count) = l.split_once(' ').ok_or(ParseError::ExpectedWhitespace)?;

                let count = count.parse()?;

                match command {
                    "forward" => Ok(Command::Forward(count)),
                    "down" => Ok(Command::Down(count)),
                    "up" => Ok(Command::Up(count)),
                    _ => Err(ParseError::UnknownCommand(command)),
                }
            })
            .collect()
    }

    fn part1(input: &[Command]) -> Self::P1 {
        let (f, d) = input.iter().fold((0, 0), |(x, y), comm| match comm {
            Command::Down(cy) => (x, y + cy),
            Command::Up(cy) => (x, y - cy),
            Command::Forward(cx) => (x + cx, y),
        });

        f * d
    }

    fn part2(input: &[Command]) -> Self::P2 {
        let (f, d, _) = input.iter().fold((0, 0, 0), |(x, y, a), comm| match comm {
            Command::Down(ca) => (x, y, a + ca),
            Command::Up(ca) => (x, y, a - ca),
            Command::Forward(cx) => (x + cx, y + a * cx, a),
        });

        f * d
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseError<'a> {
    UnknownCommand(&'a str),
    ExpectedWhitespace,
    NotAnInt(ParseIntError),
}

impl<'a> From<ParseIntError> for ParseError<'a> {
    fn from(err: ParseIntError) -> Self {
        Self::NotAnInt(err)
    }
}

use std::{error, fmt};

impl<'a> fmt::Display for ParseError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownCommand(command) => write!(f, "unknown command \"{}\"", command),
            Self::ExpectedWhitespace => write!(f, "expected whitespace"),
            Self::NotAnInt(parse_int_err) => write!(f, "{}", parse_int_err),
        }
    }
}

impl<'a> error::Error for ParseError<'a> {
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

    const INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn test() {
        assert_eq!(super::Day02::solve(INPUT), Ok((150, 900)));
    }
}