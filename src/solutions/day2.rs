use crate::solution::Solution;

pub struct Day2;

enum Result {
    Win,
    Lose,
    Draw,
}

impl Result {
    fn from_string(s: &str) -> Result {
        match s {
            "X" => Result::Lose,
            "Y" => Result::Draw,
            "Z" => Result::Win,
            _ => panic!("Unknown result: {}", s),
        }
    }

    fn points(&self) -> i32 {
        match *self {
            Result::Lose => 0,
            Result::Draw => 3,
            Result::Win => 6,
        }
    }

    fn determine_from_tupple(tupple: (&Choice, &Choice)) -> Result {
        match tupple {
            (Choice::Rock, Choice::Rock) => Result::Draw,
            (Choice::Rock, Choice::Paper) => Result::Win,
            (Choice::Rock, Choice::Scissors) => Result::Lose,
            (Choice::Paper, Choice::Rock) => Result::Lose,
            (Choice::Paper, Choice::Paper) => Result::Draw,
            (Choice::Paper, Choice::Scissors) => Result::Win,
            (Choice::Scissors, Choice::Rock) => Result::Win,
            (Choice::Scissors, Choice::Paper) => Result::Lose,
            (Choice::Scissors, Choice::Scissors) => Result::Draw,
        }
    }
}

enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn from_string(s: &str) -> Choice {
        match s {
            "A" | "X" => Choice::Rock,
            "B" | "Y" => Choice::Paper,
            "C" | "Z" => Choice::Scissors,
            _ => panic!("Invalid choice"),
        }
    }

    fn wins_against(&self) -> Choice {
        return match *self {
            Choice::Rock => Choice::Scissors,
            Choice::Paper => Choice::Rock,
            Choice::Scissors => Choice::Paper,
        };
    }

    fn loses_against(&self) -> Choice {
        return match *self {
            Choice::Rock => Choice::Paper,
            Choice::Paper => Choice::Scissors,
            Choice::Scissors => Choice::Rock,
        };
    }

    fn bonus_points(&self) -> i32 {
        return match *self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        };
    }
}


impl Solution for Day2 {
    fn part1(&self, input: &str) -> String {
        let mut score = 0;
        for line in input.lines() {
            let mut parts = line.split(' ');
            let player1 = parts.next().map(Choice::from_string).unwrap();
            let player2 = parts.next().map(Choice::from_string).unwrap();

            score += Result::determine_from_tupple((&player1, &player2)).points();
            score += player2.bonus_points();
        }
        return score.to_string();
    }

    fn part2(&self, input: &str) -> String {
        let mut score = 0;
        for line in input.lines() {
            let mut parts = line.split(' ');
            let player1 = parts.next().map(Choice::from_string).unwrap();
            let wanted_result = parts.next().map(Result::from_string).unwrap();

            score += match wanted_result {
                Result::Win => Result::Win.points() + player1.loses_against().bonus_points(),
                Result::Lose => Result::Lose.points() + player1.wins_against().bonus_points(),
                Result::Draw => Result::Draw.points() + player1.bonus_points(),
            };
        }
        return score.to_string();
    }
}
