use crate::solution::Solution;

pub struct Day1;

fn calc_for_top_n(input: &str, n: usize) -> Vec<i32> {
    let mut total_calories: Vec<i32> = Vec::new();
    let mut sum_for_elf = 0;
    for line in input.lines() {
        if line.is_empty() {
            total_calories.push(sum_for_elf);
            sum_for_elf = 0;
            continue;
        }
        sum_for_elf += line.parse::<i32>().unwrap();
    }

    total_calories.sort();
    total_calories.reverse();
    return total_calories[0..n].to_vec();
}

impl Solution for Day1 {
    fn part1(&self, input: &str) -> String {
        return calc_for_top_n(input, 1)[0].to_string();
    }

    fn part2(&self, _input: &str) -> String {
        return calc_for_top_n(_input, 3).iter().sum::<i32>().to_string();
    }
}
