use crate::solution::Solution;

pub struct Day4;

struct SectionAssignment {
    elf1_start: i32,
    elf1_end: i32,
    elf2_start: i32,
    elf2_end: i32,
}

impl SectionAssignment {
    fn new(assignment_string: &str) -> SectionAssignment {
        let assignments = assignment_string.split(',').collect::<Vec<&str>>();
        let elf1 = assignments[0].split('-').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let elf2 = assignments[1].split('-').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        return SectionAssignment {
            elf1_start: elf1[0],
            elf1_end: elf1[1],
            elf2_start: elf2[0],
            elf2_end: elf2[1],
        };
    }

    fn has_fully_overlapping_assignments(&self) -> bool {
        return self.elf1_start <= self.elf2_start && self.elf1_end >= self.elf2_end ||
            self.elf2_start <= self.elf1_start && self.elf2_end >= self.elf1_end;
    }

    fn has_any_kid_of_overlap(&self) -> bool {
        return self.elf1_start <= self.elf2_start && self.elf1_end >= self.elf2_start ||
            self.elf2_start <= self.elf1_start && self.elf2_end >= self.elf1_start;
    }
}

impl Solution for Day4 {
    fn part1(&self, input: &str) -> String {
        input.lines()
            .map(SectionAssignment::new)
            .filter(SectionAssignment::has_fully_overlapping_assignments)
            .count()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        input.lines()
            .map(SectionAssignment::new)
            .filter(SectionAssignment::has_any_kid_of_overlap)
            .count()
            .to_string()
    }
}
