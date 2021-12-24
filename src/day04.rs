use std::{cell::Cell, num::ParseIntError};

use advent_of_code::day;

day!(04);

const SIDE_LEN: usize = 5;

#[derive(Debug, Clone)]
struct BoardCell {
    value: Cell<u8>,
    marked: Cell<bool>,
}

impl BoardCell {
    fn new(n: u8) -> Self {
        Self {
            value: Cell::new(n),
            marked: Cell::new(false),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Board<const LEN: usize> {
    inner: [[BoardCell; LEN]; LEN],
}

impl<const LEN: usize> Board<LEN> {
    fn draw(&self, num: u8) {
        self.inner
            .iter()
            .flatten()
            .filter(|c| c.value.get() == num)
            .for_each(|c| c.marked.set(true));
    }

    fn sum_unmarked(&self) -> u32 {
        self.inner
            .iter()
            .flatten()
            .filter(|c| !c.marked.get())
            .map(|c| c.value.get() as u32)
            .sum()
    }

    fn has_won(&self) -> bool {
        let any_row = self
            .inner
            .iter()
            .any(|row| row.iter().all(|c| c.marked.get()));
        let any_col = (0..SIDE_LEN).any(|col| {
            self.inner
                .iter()
                .map(|row| &row[col])
                .all(|c| c.marked.get())
        });

        any_row || any_col
    }

    fn score(&self, num: u8) -> Option<u32> {
        self.draw(num);
        if self.has_won() {
            Some(self.sum_unmarked() * num as u32)
        } else {
            None
        }
    }
}

impl<const LEN: usize> From<[[BoardCell; LEN]; LEN]> for Board<LEN> {
    fn from(inner: [[BoardCell; LEN]; LEN]) -> Self {
        Board { inner }
    }
}

impl<const LEN: usize> TryFrom<&str> for Board<LEN> {
    type Error = ParseError;

    fn try_from(board: &str) -> Result<Self, Self::Error> {
        Ok(Board::<LEN>::from(
            board
                .split('\n')
                .map(|row| {
                    row.split_ascii_whitespace()
                        .map(|n| n.parse().map(BoardCell::new).map_err(ParseError::from))
                        .collect::<Result<Vec<_>, _>>()
                        .map(|vec| <[_; LEN]>::try_from(vec).map_err(ParseError::from))
                        .flatten()
                })
                .collect::<Result<Vec<_>, _>>()
                .map(|vec| <[_; LEN]>::try_from(vec).map_err(ParseError::from))
                .flatten()?,
        ))
    }
}

impl<'a> advent_of_code::Solution<'a> for Day04 {
    type Input = Vec<(u8, Box<[Board<SIDE_LEN>]>)>;
    type ParseError = ParseError;

    type P1 = Option<u32>;
    type P2 = Option<u32>;

    fn parse(input: &'a str) -> Result<Self::Input, Self::ParseError> {
        let (draws, boards) = input
            .trim()
            .split_once("\n\n")
            .ok_or(ParseError::MissingDrawNumbers)?;

        let boards = boards
            .split("\n\n")
            .map(|board| {
                board
                    .split('\n')
                    .map(|row| {
                        row.split_ascii_whitespace()
                            .map(|n| n.parse().map(BoardCell::new).map_err(ParseError::from))
                            .collect::<Result<Vec<_>, _>>()
                            .map(|vec| <[_; 5]>::try_from(vec).map_err(ParseError::from))
                            .flatten()
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .map(|vec| <[_; 5]>::try_from(vec).map_err(ParseError::from))
                    .flatten()
                    .map(|board| Board { inner: board })
            })
            .collect::<Result<Vec<Board<SIDE_LEN>>, _>>()?;

        let draws = draws
            .split(',')
            .map(|n| {
                n.parse::<u8>()
                    .map(|num| {
                        boards.iter().for_each(|b| b.draw(num));
                        (num, boards.clone().into_boxed_slice())
                    })
                    .map_err(ParseError::from)
            })
            .collect::<Result<Vec<_>, _>>();

        draws
    }

    fn part1(input: &[(u8, Box<[Board<SIDE_LEN>]>)]) -> Self::P1 {
        input
            .iter()
            .find(|(_, boards)| boards.iter().any(|board| board.has_won()))
            .map(|(draw, boards)| boards.iter().find_map(|board| board.score(*draw)))
            .flatten()
    }

    fn part2(input: &[(u8, Box<[Board<SIDE_LEN>]>)]) -> Self::P2 {
        input
            .array_windows::<2>()
            .find_map(|[(_, previous_boards), (num, boards)]| {
                if boards.iter().filter(|board| !board.has_won()).count() == 0 {
                    previous_boards
                        .iter()
                        .find(|board| !board.has_won())?
                        .score(*num)
                } else {
                    None
                }
            })
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    MissingDrawNumbers,
    NotAnInt(ParseIntError),
    WrongSideLen(usize),
}

impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> Self {
        Self::NotAnInt(err)
    }
}

impl<T> From<Vec<T>> for ParseError {
    fn from(vec: Vec<T>) -> Self {
        Self::WrongSideLen(vec.len())
    }
}

use std::{error, fmt};

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingDrawNumbers => write!(f, "draw numbers are missing"),
            Self::NotAnInt(parse_int_err) => write!(f, "{}", parse_int_err),
            Self::WrongSideLen(actual_len) => write!(
                f,
                "expected {} as the side length but instead found {}",
                SIDE_LEN, actual_len
            ),
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

    const INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn test() {
        assert_eq!(super::Day04::solve(INPUT), Ok((Some(4512), Some(1924))));
    }
}
