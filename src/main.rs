#![feature(array_windows)]

use crate::solution::Solution;

mod solution;

mod day01;

const INPUT: [&str; 1] = [include_str!("../input/01.txt")];

fn main() {
    let mut children = Vec::with_capacity(32);

    children.push(day01::Day1::run(INPUT[0], 1233, 1275));

    let (_, parallel_dur) = solution::bench(move || {
        for child in children {
            let _handle = child.join();
        }
    });

    println!(
        "\nTotal (parallel): {}ms",
        solution::format_dur(parallel_dur)
    );
}
