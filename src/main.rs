#[macro_use(scan_fmt)]
extern crate scan_fmt;
// #[macro_use(lazy_static)]
// extern crate lazy_static;

use aoc_main;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

aoc_main::main! {
    year 2020;
    day01 : generator => part_one, part_two;
    day02 : generator => part_one, part_two;
    day03 : generator => part_one, part_two;
    day04 : generator => part_one, part_two;
    day05 : generator => part_one, part_two;
    day06 : generator => part_one, part_two;
    day07 : generator => part_one, part_two;
    day08 : generator => part_one, part_two;
}