use std::{error::Error, fmt::Display};

pub trait Solution {
    type Input;
    type ParseError: Error;

    type P1: Display;
    type P2: Display;

    fn parse(input: &str) -> Result<Self::Input, Self::ParseError>;

    fn part1(input: &Self::Input) -> Self::P1;

    fn part2(input: &Self::Input) -> Self::P2;

    fn solve(input: &str) -> Result<(Self::P1, Self::P2), Self::ParseError> {
        let input = Self::parse(input)?;

        let p1 = Self::part1(&input);
        let p2 = Self::part2(&input);

        Ok((p1, p2))
    }
}
