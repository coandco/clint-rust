#[macro_use(scan_fmt)]
extern crate scan_fmt;
// #[macro_use(lazy_static)]
// extern crate lazy_static;

// use aoc_main;
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
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;

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
    day09 : generator => part_one, part_two;
    day10 : generator => part_one, part_two;
    day11 : generator => part_one_vecgrid, part_one_hashgrid, part_two_vecgrid, part_two_hashgrid;
    day12 : generator => part_one, part_two;
    day13 : generator => part_one, part_two;
    day14 : generator => part_one, part_two;
    day15 : generator => part_one, part_two;
    day16 : generator => part_one, part_two;
    day17 : generator => part_one, part_two;
    day18 : generator => part_one, part_two;
    day19 : generator => part_one, part_two;
}