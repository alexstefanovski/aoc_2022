use crate::solution::Solution;

pub struct Day3;

fn prio_score(char: &char) -> i32 {
    return match char {
        'a'..='z' => *char as u8 - 'a' as u8 + 1,
        'A'..='Z' => *char as u8 - 'A' as u8 + 27,
        _ => 0,
    } as i32;
}

impl Solution for Day3 {
    fn part1(&self, input: &str) -> String {
        let mut sum = 0;

        for line in input.lines() {
            let (comp1, comp2) = line.split_at(line.len() / 2);
            for char in comp1.chars() {
                if comp2.contains(char) {
                    sum += prio_score(&char);
                    break;
                }
            }
        }

        sum.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut sum = 0;
        (0..input.lines().count()).step_by(3).for_each(|i| {
            let r1 = input.lines().nth(i).unwrap();
            let r2 = input.lines().nth(i + 1).unwrap();
            let r3 = input.lines().nth(i + 2).unwrap();

            for char in r1.chars() {
                if r2.contains(char) && r3.contains(char) {
                    sum += prio_score(&char);
                    break;
                }
            }
        });

        sum.to_string()
    }
}
