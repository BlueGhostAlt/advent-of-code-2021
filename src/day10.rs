use std::str;

advent_of_code::day!(10);

#[derive(Debug)]
pub enum Line {
    Corrupt(Token),
    Incomplete(Vec<Token>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Token {
    inner: char,
}

impl Token {
    fn is_opener(&self) -> bool {
        matches!(self.inner, '(' | '[' | '{' | '<')
    }

    fn matching_closer(&self) -> Option<Token> {
        if self.is_opener() {
            let inner = match self.inner {
                '(' => ')',
                '[' => ']',
                '{' => '}',
                '<' => '>',
                _ => unreachable!(),
            };

            Some(Token { inner })
        } else {
            None
        }
    }
}

impl str::FromStr for Line {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s
            .chars()
            .map(Token::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        let line = tokens
            .iter()
            .copied()
            .try_fold(Vec::new(), |mut stack, t| {
                if t.is_opener() {
                    stack.push(t);
                    Ok(stack)
                } else if stack
                    .last()
                    .and_then(|top| top.matching_closer())
                    .contains(&t)
                {
                    stack.pop();
                    Ok(stack)
                } else {
                    Err(t)
                }
            })
            .map(|openers| {
                openers
                    .iter()
                    .rev()
                    .map(|t| {
                        t.matching_closer()
                            .ok_or(ParseError::UnexpectedCloser(t.inner))
                    })
                    .collect::<Result<Vec<_>, _>>()
            });

        let line = match line {
            Ok(ts) => Line::Incomplete(ts?),
            Err(t) => Line::Corrupt(t),
        };

        Ok(line)
    }
}

impl Into<char> for Token {
    fn into(self) -> char {
        self.inner as _
    }
}

impl TryFrom<char> for Token {
    type Error = ParseError;

    fn try_from(inner: char) -> Result<Self, Self::Error> {
        match inner {
            '(' | ')' | '[' | ']' | '{' | '}' | '<' | '>' => Ok(Token { inner }),
            _ => Err(ParseError::InvalidCharacter(inner)),
        }
    }
}

impl advent_of_code::Solution<'_> for Day10 {
    type Input = Vec<Line>;
    type ParseError = ParseError;

    type P1 = u32;
    type P2 = u64;

    fn parse(input: &str) -> Result<Self::Input, Self::ParseError> {
        input.lines().map(str::parse).collect()
    }

    fn part1(input: &[Line]) -> Self::P1 {
        input
            .iter()
            .filter_map(|l| match l {
                Line::Corrupt(t) => match t.inner {
                    ')' => Some(3),
                    ']' => Some(57),
                    '}' => Some(1197),
                    '>' => Some(25137),
                    _ => None,
                },
                Line::Incomplete(_) => None,
            })
            .sum()
    }

    fn part2(input: &[Line]) -> Self::P2 {
        let mut scores = input
            .iter()
            .filter_map(|l| match l {
                Line::Corrupt(_) => None,
                Line::Incomplete(tokens) => tokens
                    .iter()
                    .filter_map(|t| match t.inner {
                        ')' => Some(1),
                        ']' => Some(2),
                        '}' => Some(3),
                        '>' => Some(4),
                        _ => None,
                    })
                    .reduce(|score, t| score * 5 + t),
            })
            .collect::<Vec<_>>();

        scores.sort_unstable();

        scores[scores.len() / 2]
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidCharacter(char),
    UnexpectedCloser(char),
}

use std::{error, fmt};

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCharacter(c) => write!(f, "invalid character {}", c),
            Self::UnexpectedCloser(c) => write!(f, "unexpected closer {}", c),
        }
    }
}

impl error::Error for ParseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use advent_of_code::Solution;

    const INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test() {
        assert_eq!(super::Day10::solve(INPUT), Ok((26_397, 288_957)));
    }
}
