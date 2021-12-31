use std::num::ParseIntError;

advent_of_code::day!(07);

type ParseError = ParseIntError;
type Position = i32;

fn compute_fuel_cost<F>(positions: &[Position], steps_to_cost: F) -> Option<i32>
where
    F: Fn(i32) -> i32,
{
    let min = *positions.iter().min()?;
    let max = *positions.iter().max()?;

    (min..=max)
        .map(|t| {
            positions
                .iter()
                .map(|&x| {
                    let steps = i32::abs(x - t);

                    steps_to_cost(steps)
                })
                .sum()
        })
        .min()
}

impl advent_of_code::Solution<'_> for Day07 {
    type Input = Vec<Position>;
    type ParseError = ParseError;

    type P1 = Option<i32>;
    type P2 = Option<i32>;

    fn parse(input: &str) -> Result<Self::Input, Self::ParseError> {
        input.trim().split(',').map(str::parse).collect()
    }

    fn part1(input: &[Position]) -> Self::P1 {
        compute_fuel_cost(input, |s| s)
    }

    fn part2(input: &[Position]) -> Self::P2 {
        compute_fuel_cost(input, |s| s * (s + 1) / 2)
    }
}

#[cfg(test)]
mod tests {
    use advent_of_code::Solution;

    const INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test() {
        assert_eq!(super::Day07::solve(INPUT), Ok((Some(37), Some(168))));
    }
}
