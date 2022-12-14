use crate::Move::{Paper, Rock, Scissor};
use crate::Outcome::{Draw, Lose, Win};

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .into_iter()
            .map(|line| {
                let mut chars = line.split(' ');
                let opponent = Move::from(chars.next().unwrap().chars().next().unwrap());
                let me = Move::from(chars.next().unwrap().chars().next().unwrap());
                let outcome = Move::get_outcome(me, opponent);
                Move::get_points(me, outcome)
            })
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .into_iter()
            .map(|line| {
                let mut chars = line.split(' ');
                let opponent = Move::from(chars.next().unwrap().chars().next().unwrap());
                let me = Move::move_should_be(
                    opponent,
                    Move::parse_intended_outcome(chars.next().unwrap().chars().next().unwrap()),
                );
                let outcome = Move::get_outcome(me, opponent);
                Move::get_points(me, outcome)
            })
            .sum::<u32>(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[derive(Clone, Copy, Debug)]
enum Move {
    Rock,
    Paper,
    Scissor,
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

impl From<char> for Move {
    fn from(value: char) -> Self {
        match value {
            'X' => Rock,
            'Y' => Paper,
            'Z' => Scissor,

            'A' => Rock,
            'B' => Paper,
            'C' => Scissor,
            _ => {
                unreachable!()
            }
        }
    }
}

impl Move {
    fn parse_intended_outcome(c: char) -> Outcome {
        match c {
            'X' => Lose,
            'Y' => Draw,
            'Z' => Win,
            _ => {
                unreachable!()
            }
        }
    }
    fn move_should_be(opponent_move: Move, intended_outcome: Outcome) -> Move {
        match opponent_move {
            Rock => match intended_outcome {
                Win => Paper,
                Lose => Scissor,
                Draw => Rock,
            },
            Paper => match intended_outcome {
                Win => Scissor,
                Lose => Rock,
                Draw => Paper,
            },
            Scissor => match intended_outcome {
                Win => Rock,
                Lose => Paper,
                Draw => Scissor,
            },
        }
    }
    fn get_outcome(my_move: Move, opponent_move: Move) -> Outcome {
        match my_move {
            Rock => match opponent_move {
                Rock => Draw,
                Paper => Lose,
                Scissor => Win,
            },
            Paper => match opponent_move {
                Rock => Win,
                Paper => Draw,
                Scissor => Lose,
            },
            Scissor => match opponent_move {
                Rock => Lose,
                Paper => Win,
                Scissor => Draw,
            },
        }
    }
    fn get_points(my_move: Move, outcome: Outcome) -> u32 {
        let points_from_victor: u32 = match outcome {
            Win => 6,
            Draw => 3,
            Lose => 0,
        };
        let points_from_piece: u32 = match my_move {
            Rock => 1,
            Paper => 2,
            Scissor => 3,
        };
        points_from_piece + points_from_victor
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn points_are_correct() {
        let mut input = "A X";
        assert_eq!(part_one(&input), Some(4));
        input = "A Y";
        assert_eq!(part_one(&input), Some(8));
        input = "A Z";
        assert_eq!(part_one(&input), Some(3));
        input = "B X";
        assert_eq!(part_one(&input), Some(1));
        input = "B Y";
        assert_eq!(part_one(&input), Some(5));
        input = "B Z";
        assert_eq!(part_one(&input), Some(9));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
