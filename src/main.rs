#![feature(array_windows)]

mod day01;

const INPUT: [&str; 1] = [include_str!("../input/01.txt")];

fn main() {
    assert_eq!(day01::part1(INPUT[0]), 1233);
    assert_eq!(day01::part2(INPUT[0]), 1275);
}
