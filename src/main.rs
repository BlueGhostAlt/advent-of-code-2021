#![feature(
    array_windows,
    never_type,
    result_flattening,
    result_into_ok_or_err,
    option_result_contains
)]

use advent_of_code::{Day, Solution};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

fn main() {
    advent_of_code::days!(
        01 => (1_233, 1_275),
        02 => (1_882_980, 1_971_232_560),
        03 => (Some(1_071_734), Some(6_124_992)),
        04 => (Some(49_860), Some(24_628)),
        05 => (6_856, 20_666),
        06 => (355_386, 1_613_415_325_809),
        07 => (Some(342_730), Some(92_335_207)),
        08 => (470, Some(989_396)),
        09 => (566, 891_684),
        10 => (339_411, 2_289_754_624)
    );
}
