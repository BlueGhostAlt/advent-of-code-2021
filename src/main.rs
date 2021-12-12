#![feature(array_windows)]

use crate::solution::Solution;

mod solution;

mod day01;

const INPUT: [&str; 1] = [include_str!("../input/01.txt")];

fn main() {
    assert_eq!(day01::Day1::solve(INPUT[0]), Ok((1233, 1275)));
}
