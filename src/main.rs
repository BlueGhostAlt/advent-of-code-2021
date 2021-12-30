#![feature(array_windows, never_type, result_flattening)]

use std::thread::JoinHandle;

use advent_of_code::{Day, Solution};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

fn main() {
    let mut children = Vec::<JoinHandle<()>>::with_capacity(32);

    children.push(day01::Day01::run(day01::Day01::input(), 1_233, 1_275));
    children.push(day02::Day02::run(
        day02::Day02::input(),
        1_882_980,
        1_971_232_560,
    ));
    children.push(day03::Day03::run(
        day03::Day03::input(),
        1_071_734,
        6_124_992,
    ));
    children.push(day04::Day04::run(
        day04::Day04::input(),
        Some(49_860),
        Some(24_628),
    ));
    children.push(day05::Day05::run(day05::Day05::input(), 6_856, 20_666));

    let (_, parallel_dur) = advent_of_code::bench(move || {
        for child in children {
            let _handle = child.join();
        }
    });

    println!(
        "\nTotal (parallel): {}ms",
        advent_of_code::format_dur(parallel_dur)
    );
}
