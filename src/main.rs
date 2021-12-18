#![feature(array_windows, never_type)]

use std::thread::JoinHandle;

use advent_of_code::{Day, Solution};

mod day01;
mod day02;

fn main() {
    let mut children = Vec::<JoinHandle<()>>::with_capacity(32);

    children.push(day01::Day01::run(day01::Day01::input(), 1233, 1275));
    children.push(day02::Day02::run(
        day02::Day02::input(),
        1882980,
        1971232560,
    ));

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
