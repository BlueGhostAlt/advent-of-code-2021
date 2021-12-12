pub fn part1(input: &str) -> usize {
    let nums = input
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    nums.array_windows::<2>()
        .map(|[x1, x2]| x2 > x1)
        .filter(|x| *x)
        .count()
}

pub fn part2(input: &str) -> usize {
    let nums = input
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let window_sums = nums
        .array_windows::<3>()
        .map(|w| w.iter().sum::<i32>())
        .collect::<Vec<_>>();

    window_sums
        .array_windows::<2>()
        .map(|[x1, x2]| x2 > x1)
        .filter(|x| *x)
        .count()
}

#[cfg(test)]
mod tests {

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
    fn part1() {
        assert_eq!(super::part1(INPUT), 7);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 5);
    }
}
