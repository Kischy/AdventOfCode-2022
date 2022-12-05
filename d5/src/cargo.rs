use regex;
use std::collections::VecDeque;

pub struct Cargo {
    crates: VecDeque<VecDeque<char>>,
}

fn add_cargo_line(crates: &mut VecDeque<VecDeque<char>>, crate_line_descr: &Vec<char>) {
    for i in (0usize..crate_line_descr.len()).step_by(4) {
        if crate_line_descr.len() <= i + 1 {
            break;
        }

        if crate_line_descr[i + 1].is_alphabetic() {
            let vec_index = i / 3;
            while vec_index >= crates.len() {
                crates.push_back(VecDeque::new());
            }
            crates[vec_index].push_front(crate_line_descr[i + 1]);
        }
    }
}

struct MoveValues {
    quantity: usize,
    from_index: usize,
    to_index: usize,
}

fn get_move_values(move_line: &str) -> MoveValues {
    let re = regex::Regex::new(r"[0-9]+").unwrap();
    let parts: Vec<usize> = re
        .find_iter(move_line)
        .filter_map(|digits| digits.as_str().parse().ok())
        .collect();

    MoveValues {
        quantity: parts[0],
        from_index: parts[1] - 1,
        to_index: parts[2] - 1,
    }
}

impl Cargo {
    pub fn from_string(cargo: &str) -> Cargo {
        let mut crates: VecDeque<VecDeque<char>> = VecDeque::new();

        for line in cargo.lines() {
            let l_by: Vec<char> = line.chars().collect();
            add_cargo_line(&mut crates, &l_by);
        }

        Cargo { crates: crates }
    }

    pub fn get_crates(&self) -> &VecDeque<VecDeque<char>> {
        &self.crates
    }

    pub fn get_top_crates(&self) -> String {
        let mut top_crates = String::new();
        for pile in &self.crates {
            if pile.is_empty() {
                continue;
            }
            top_crates.push(pile.back().unwrap().clone());
        }
        top_crates
    }

    pub fn move_crates(&mut self, move_line: &str) {
        let move_val = get_move_values(move_line);
        for _ in 0..move_val.quantity {
            if let Some(cr) = self.crates[move_val.from_index].pop_back() {
                self.crates[move_val.to_index].push_back(cr);
            }
        }
    }
}

#[cfg(test)]
mod cargo_tests {
    use super::get_move_values;
    use super::Cargo;
    use indoc::indoc;
    use std::collections::VecDeque;

    #[test]
    fn from_string_tests() {
        let cargo_input = indoc! {"
            [D]
        [N] [C]    
        [Z] [M] [P]
         1   2   3 
        
         "};
        let cargo = Cargo::from_string(cargo_input);
        let crates = &cargo.crates;
        assert_eq!(crates.len(), 3);
        assert_eq!(crates[0], VecDeque::from(vec!['Z', 'N']));
        assert_eq!(crates[1], VecDeque::from(vec!['M', 'C', 'D']));
        assert_eq!(crates[2], VecDeque::from(vec!['P']));
    }

    #[test]
    fn get_move_values_test() {
        let one_move = get_move_values("move 31 from 13 to 333");
        assert_eq!(one_move.quantity, 31);
        assert_eq!(one_move.from_index, 12);
        assert_eq!(one_move.to_index, 332);
    }

    #[test]
    fn get_top_crates_tests() {
        let mut cargo = Cargo {
            crates: VecDeque::new(),
        };
        cargo.crates.push_back(VecDeque::from(vec!['Z', 'N']));
        cargo.crates.push_back(VecDeque::from(vec!['M', 'C', 'D']));
        cargo.crates.push_back(VecDeque::from(vec!['P']));

        assert_eq!(cargo.get_top_crates(), "NDP");
    }

    #[test]
    fn move_crates_test() {
        let mut cargo = Cargo {
            crates: VecDeque::new(),
        };
        cargo.crates.push_back(VecDeque::from(vec!['Z', 'N']));
        cargo.crates.push_back(VecDeque::from(vec!['M', 'C', 'D']));
        cargo.crates.push_back(VecDeque::from(vec!['P']));

        cargo.move_crates("move 1 from 2 to 1");
        {
            let crates = &cargo.crates;
            assert_eq!(crates[0], VecDeque::from(vec!['Z', 'N', 'D']));
            assert_eq!(crates[1], VecDeque::from(vec!['M', 'C']));
            assert_eq!(crates[2], VecDeque::from(vec!['P']));
        }

        cargo.move_crates("move 3 from 1 to 3");
        {
            let crates = &cargo.crates;
            assert_eq!(crates[0], VecDeque::from(vec![]));
            assert_eq!(crates[1], VecDeque::from(vec!['M', 'C']));
            assert_eq!(crates[2], VecDeque::from(vec!['P', 'D', 'N', 'Z']));
        }

        cargo.move_crates("move 2 from 2 to 1");
        {
            let crates = &cargo.crates;
            assert_eq!(crates[0], VecDeque::from(vec!['C', 'M']));
            assert_eq!(crates[1], VecDeque::from(vec![]));
            assert_eq!(crates[2], VecDeque::from(vec!['P', 'D', 'N', 'Z']));
        }

        cargo.move_crates("move 1 from 1 to 2");
        {
            let crates = &cargo.crates;
            assert_eq!(crates[0], VecDeque::from(vec!['C']));
            assert_eq!(crates[1], VecDeque::from(vec!['M']));
            assert_eq!(crates[2], VecDeque::from(vec!['P', 'D', 'N', 'Z']));
        }
    }
}
