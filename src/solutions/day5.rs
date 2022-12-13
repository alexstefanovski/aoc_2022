use crate::solution::Solution;
use itertools::Itertools;

pub struct Day5;

#[derive(Debug)]
struct RearrangementProcedure {
    num: usize,
    from: usize,
    to: usize,
}

fn parse(input: &str) -> (Vec<Vec<char>>, Vec<RearrangementProcedure>) {
    let (initial_setup, procedures_str) = input.split_once("\n\n").unwrap();
    let (existing_stacks, platforms) = initial_setup.rsplit_once('\n').unwrap();
    let num_platforms: usize = platforms.split_whitespace().last().unwrap().parse().unwrap();
    let mut stacks = vec![<Vec<char>>::new(); num_platforms];
    let mut procedures_result: Vec<RearrangementProcedure> = vec![];

    existing_stacks.lines().for_each(|line| {
        line.chars().chunks(4).into_iter().enumerate().for_each(|(i, mut chunk)| {
            let second_char = chunk.nth(1).unwrap();
            if second_char.is_alphabetic() {
                stacks[i].push(second_char);
            }
        });
    });

    for line in procedures_str.lines() {
        let (num, other) = line
            .strip_prefix("move ").unwrap()
            .split_once(" from ").unwrap();
        let (from, to) = other.split_once(" to ").unwrap();
        procedures_result.push(RearrangementProcedure {
            num: num.parse().unwrap(),
            from: from.parse::<usize>().unwrap() - 1,
            to: to.parse::<usize>().unwrap() - 1,
        });
    }

    (stacks, procedures_result)
}


impl Solution for Day5 {
    fn part1(&self, input: &str) -> String {
        let (mut stacks, procedures) = parse(input);
        procedures.iter().for_each(|procedure| {
            [0..procedure.num].iter().for_each(|_| {
                stacks[procedure.from].pop().and_then(|item| Some(stacks[procedure.to].push(item)));
            });
        });

        stacks.iter().filter_map(|stack| stack.iter().last()).collect()
    }

    fn part2(&self, input: &str) -> String {
        let (mut stacks, procedures) = parse(input);

        procedures.iter().for_each(|procedure| {
            let initial_stack_length = stacks[procedure.from].len();
            let removed = stacks[procedure.from].split_off(initial_stack_length - procedure.num);
            stacks[procedure.to].extend(removed);
        });

        stacks.iter().filter_map(|stack| stack.iter().last()).collect()
    }
}
