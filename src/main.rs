use std::fs;

use aoc_2022::{
    solution::Solution,
    solutions::{
        day1::{Day1},
        day2::{Day2},
        day3::{Day3},
        day4::{Day4},
        day5::{Day5},
        day6::{Day6},
    },
};

fn main() {
    let days: Vec<Box<dyn Solution>> = vec![
        Box::new(Day1),
        Box::new(Day2),
        Box::new(Day3),
        Box::new(Day4),
        Box::new(Day5),
        Box::new(Day6),
    ];

    days.iter()
        .enumerate()
        .for_each(|(n, solution)| day_n((n + 1) as i32, solution));
}

fn day_n(n: i32, solution: &Box<dyn Solution>) {
    let input = fs::read_to_string(format!("src/input/day{}.txt", n)).expect("Unable to read file");
    println!("Day {} Part 1: {}, Part 2: {}", n, solution.part1(&input), solution.part2(&input));
}
