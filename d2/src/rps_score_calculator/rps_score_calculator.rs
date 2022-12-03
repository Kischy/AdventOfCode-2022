use std::collections::HashMap;

#[derive(PartialEq, Clone, Copy)]
enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}

fn get_shape_points(rps: RockPaperScissors) -> u128 {
    match rps {
        RockPaperScissors::Rock => return 1,
        RockPaperScissors::Paper => return 2,
        RockPaperScissors::Scissors => return 3,
    }
}

fn get_match_points(my_choice: RockPaperScissors, op_choice: RockPaperScissors) -> u128 {
    let choice_points = get_shape_points(my_choice);

    if my_choice == op_choice {
        return 3 + choice_points;
    }

    if my_choice == RockPaperScissors::Rock && op_choice == RockPaperScissors::Scissors {
        return 6 + choice_points;
    }

    if my_choice == RockPaperScissors::Paper && op_choice == RockPaperScissors::Rock {
        return 6 + choice_points;
    }

    if my_choice == RockPaperScissors::Scissors && op_choice == RockPaperScissors::Paper {
        return 6 + choice_points;
    }

    0 + choice_points
}

#[derive(PartialEq, Clone, Copy)]
enum LoseDrawWin {
    Lose,
    Draw,
    Win,
}

fn get_match_points_with_wanted_result(
    wanted_result: LoseDrawWin,
    op_rps: RockPaperScissors,
) -> u128 {

    match wanted_result {
        LoseDrawWin::Lose => match op_rps {
            RockPaperScissors::Rock => {
                return get_match_points(RockPaperScissors::Scissors, op_rps)
            }
            RockPaperScissors::Paper => return get_match_points(RockPaperScissors::Rock, op_rps),
            RockPaperScissors::Scissors => {
                return get_match_points(RockPaperScissors::Paper, op_rps)
            }
        },
        LoseDrawWin::Draw => return get_match_points(op_rps, op_rps),
        LoseDrawWin::Win => match op_rps {
            RockPaperScissors::Rock => return get_match_points(RockPaperScissors::Paper, op_rps),
            RockPaperScissors::Paper => {
                return get_match_points(RockPaperScissors::Scissors, op_rps)
            }
            RockPaperScissors::Scissors => {
                return get_match_points(RockPaperScissors::Rock, op_rps)
            }
        },
    }
}

pub struct RPSCalculator {
    scores_part1: HashMap<String, u128>,
    scores_part2: HashMap<String, u128>,
}

impl RPSCalculator {
    pub fn new() -> RPSCalculator {
        RPSCalculator {
            scores_part1: HashMap::new(),
            scores_part2: HashMap::new(),
        }
    }

    pub fn calculate_score_part1(&mut self, game: &str) -> u128 {
        if let Some(val) = self.scores_part1.get(game) {
            return *val;
        }

        let mut op_choice = RockPaperScissors::Paper;
        let mut my_choice = RockPaperScissors::Paper;

        for c in game.chars() {
            if c == 'A' {
                op_choice = RockPaperScissors::Rock;
            } else if c == 'B' {
                op_choice = RockPaperScissors::Paper;
            } else if c == 'C' {
                op_choice = RockPaperScissors::Scissors;
            } else if c == 'X' {
                my_choice = RockPaperScissors::Rock;
            } else if c == 'Y' {
                my_choice = RockPaperScissors::Paper;
            } else if c == 'Z' {
                my_choice = RockPaperScissors::Scissors;
            }
        }
        let score = get_match_points(my_choice, op_choice);
        self.scores_part1.insert(game.to_string(), score);
        score
    }

    pub fn calculate_score_part2(&mut self, game: &str) -> u128 {
        if let Some(val) = self.scores_part2.get(game) {
            return *val;
        }

        let mut op_choice = RockPaperScissors::Paper;
        let mut wanted_result = LoseDrawWin::Lose;

        for c in game.chars() {
            if c == 'A' {
                op_choice = RockPaperScissors::Rock;
            } else if c == 'B' {
                op_choice = RockPaperScissors::Paper;
            } else if c == 'C' {
                op_choice = RockPaperScissors::Scissors;
            } else if c == 'X' {
                wanted_result = LoseDrawWin::Lose
            } else if c == 'Y' {
                wanted_result = LoseDrawWin::Draw
            } else if c == 'Z' {
                wanted_result = LoseDrawWin::Win
            }
        }
        let score = get_match_points_with_wanted_result(wanted_result, op_choice);
        self.scores_part2.insert(game.to_string(), score);
        score
    }
}

#[cfg(test)]
mod tests {
    use crate::rps_score_calculator::rps_score_calculator::RPSCalculator;

    #[test]
    fn calculate_score_part1_test() {
        let mut rps_calc = RPSCalculator::new();
        assert_eq!(rps_calc.calculate_score_part1("A Y"), 2 + 6);
        assert_eq!(rps_calc.calculate_score_part1("B X"), 1 + 0);
        assert_eq!(rps_calc.calculate_score_part1("C Z"), 3 + 3);
        assert_eq!(rps_calc.calculate_score_part1("A Y"), 2 + 6);
        assert_eq!(rps_calc.calculate_score_part1("B X"), 1 + 0);
        assert_eq!(rps_calc.calculate_score_part1("C Z"), 3 + 3);
    }

    #[test]
    fn calculate_score_part2_test() {
        let mut rps_calc = RPSCalculator::new();
        assert_eq!(rps_calc.calculate_score_part2("A Y"), 1 + 3);
        assert_eq!(rps_calc.calculate_score_part2("B X"), 1 + 0);
        assert_eq!(rps_calc.calculate_score_part2("C Z"), 1 + 6);
        assert_eq!(rps_calc.calculate_score_part2("A Y"), 1 + 3);
        assert_eq!(rps_calc.calculate_score_part2("B X"), 1 + 0);
        assert_eq!(rps_calc.calculate_score_part2("C Z"), 1 + 6);
    }
}
