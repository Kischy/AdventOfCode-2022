use std::collections::HashMap;

pub struct RPSCalculator {
    scores: HashMap<String, u128>,
}

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

fn get_match_points(my_rps: RockPaperScissors, op_rps: RockPaperScissors) -> u128 {
    if my_rps == op_rps {
        return 3;
    }

    if my_rps == RockPaperScissors::Rock && op_rps == RockPaperScissors::Scissors {
        return 6;
    }

    if my_rps == RockPaperScissors::Paper && op_rps == RockPaperScissors::Rock {
        return 6;
    }

    if my_rps == RockPaperScissors::Scissors && op_rps == RockPaperScissors::Paper {
        return 6;
    }

    0
}

impl RPSCalculator {
    pub fn new() -> RPSCalculator {
        RPSCalculator {
            scores: HashMap::new(),
        }
    }

    pub fn calculate_score(&mut self, game: &str) -> u128 {
        if let Some(val) = self.scores.get(game) {
            return *val;
        }

        let mut oponent_selection = RockPaperScissors::Paper;
        let mut my_selection = RockPaperScissors::Paper;

        for c in game.chars() {
            if c == 'A' {
                oponent_selection = RockPaperScissors::Rock;
            } else if c == 'B' {
                oponent_selection = RockPaperScissors::Paper;
            } else if c == 'C' {
                oponent_selection = RockPaperScissors::Scissors;
            } else if c == 'X' {
                my_selection = RockPaperScissors::Rock;
            } else if c == 'Y' {
                my_selection = RockPaperScissors::Paper;
            } else if c == 'Z' {
                my_selection = RockPaperScissors::Scissors;
            }
        }
        let score =
            get_shape_points(my_selection) + get_match_points(my_selection, oponent_selection);
        self.scores.insert(game.to_string(), score);
        score
    }
}

#[cfg(test)]
mod tests {
    use crate::rps_score_calculator::rps_score_calculator::RPSCalculator;

    #[test]
    fn get_fattest_elf_test() {
        let mut rps_calc = RPSCalculator::new();
        assert_eq!(rps_calc.calculate_score("A Y"), 2 + 6);
        assert_eq!(rps_calc.calculate_score("B X"), 1 + 0);
        assert_eq!(rps_calc.calculate_score("C Z"), 3 + 3);
        assert_eq!(rps_calc.calculate_score("A Y"), 2 + 6);
        assert_eq!(rps_calc.calculate_score("B X"), 1 + 0);
        assert_eq!(rps_calc.calculate_score("C Z"), 3 + 3);
    }
}
