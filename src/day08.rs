advent_of_code::day!(08);

type Pattern<'a> = &'a str;
type Output<'a> = &'a str;

impl<'a> advent_of_code::Solution<'a> for Day08 {
    type Input = Vec<(Pattern<'a>, Output<'a>)>;
    type ParseError = ParseError;

    type P1 = usize;
    type P2 = Option<u32>;

    fn parse(input: &'a str) -> Result<Self::Input, Self::ParseError> {
        input
            .trim()
            .lines()
            .map(|l| {
                let (patterns, output) =
                    l.split_once('|').ok_or(ParseError::MissingPipeSeparator)?;

                Ok((patterns.trim(), output.trim()))
            })
            .collect()
    }

    fn part1(input: &[(Pattern<'a>, Output<'a>)]) -> Self::P1 {
        input
            .iter()
            .flat_map(|(_, output)| output.split_ascii_whitespace())
            .filter(|output| matches!(output.len(), 2 | 3 | 4 | 7))
            .count()
    }

    fn part2(input: &[(Pattern<'a>, Output<'a>)]) -> Self::P2 {
        input
            .iter()
            .map(|(patterns, output)| {
                let one = patterns
                    .split_ascii_whitespace()
                    .find(|pattern| pattern.len() == 2)?;
                let four = patterns
                    .split_ascii_whitespace()
                    .find(|pattern| pattern.len() == 4)?;

                let digits = output
                    .split_ascii_whitespace()
                    .map(|output| match output.len() {
                        2 => Some(1),
                        3 => Some(7),
                        4 => Some(4),
                        7 => Some(8),
                        len => match (
                            len,
                            output.bytes().filter(|&c| one.contains(c as char)).count(),
                            output.bytes().filter(|&c| four.contains(c as char)).count(),
                        ) {
                            (5, 1, 3) => Some(5),
                            (5, 2, 3) => Some(3),
                            (5, _, 2) => Some(2),
                            (6, 1, _) => Some(6),
                            (6, _, 3) => Some(0),
                            (6, _, 4) => Some(9),
                            _ => None,
                        },
                    });

                digits
                    .enumerate()
                    .try_fold(0, |acc, (i, digit)| match digit {
                        Some(digit) => Some(acc + digit * 10u32.pow(3 - i as u32)),
                        None => digit,
                    })
            })
            .sum()
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    MissingPipeSeparator,
}

use std::{error, fmt};

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingPipeSeparator => write!(f, "missing pipe separator"),
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

    const INPUT: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test() {
        assert_eq!(super::Day08::solve(INPUT), Ok((26, Some(61_229))));
    }
}
