use std::num::ParseIntError;

use advent_of_code::day;

day!(01);

impl advent_of_code::Solution<'_> for Day01 {
    type Input = Vec<i32>;
    type ParseError = ParseIntError;

    type P1 = usize;
    type P2 = usize;

    fn parse(input: &str) -> Result<Self::Input, Self::ParseError> {
        input.lines().map(|l| l.parse()).collect()
    }

    fn part1(input: &[i32]) -> Self::P1 {
        input
            .array_windows::<2>()
            .filter(|[x1, x2]| x2 > x1)
            .count()
    }

    fn part2(input: &[i32]) -> Self::P2 {
        input
            .array_windows::<4>()
            .filter(|[a, _, _, d]| d > a)
            .count()
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
        assert_eq!(super::Day01::solve(INPUT), Ok((7, 5)));
    }
}
