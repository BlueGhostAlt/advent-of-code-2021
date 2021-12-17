use std::num::ParseIntError;

use advent_of_code::Solution;

pub struct Day1;

impl Solution for Day1 {
    type Input = Vec<i32>;
    type ParseError = ParseIntError;

    type P1 = usize;
    type P2 = usize;

    fn parse(input: &str) -> Result<Self::Input, Self::ParseError> {
        input.lines().map(|l| l.parse()).collect()
    }

    fn part1(input: &Self::Input) -> Self::P1 {
        input
            .array_windows::<2>()
            .filter(|[x1, x2]| x2 > x1)
            .count()
    }

    fn part2(input: &Self::Input) -> Self::P2 {
        let window_sums = input
            .array_windows::<3>()
            .map(|w| w.iter().sum::<i32>())
            .collect::<Vec<_>>();

        window_sums
            .array_windows::<2>()
            .filter(|[x1, x2]| x2 > x1)
            .count()
    }

    fn day() -> usize {
        1
    }
}

#[cfg(test)]
mod tests {

    use advent_of_code::Solution;

    const INPUT: &str = "199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn test() {
        assert_eq!(super::Day1::solve(INPUT), Ok((7, 5)));
    }
}
