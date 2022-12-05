use regex;
use std::collections::VecDeque;

fn add_cargo_line(crates: &mut VecDeque<VecDeque<char>>, crate_line_descr: &Vec<char>) {
    let step = 4;
    for i in (0usize..crate_line_descr.len()).step_by(step) {
        if crate_line_descr.len() <= i + 1 {
            break;
        }

        if crate_line_descr[i + 1].is_alphabetic() {
            let vec_index = i / step;
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

#[derive(Clone)]
pub struct Cargo {
    crates: VecDeque<VecDeque<char>>,
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

    pub fn move_crates_9000(&mut self, move_line: &str) {
        let move_val = get_move_values(move_line);
        for _ in 0..move_val.quantity {
            if let Some(cr) = self.crates[move_val.from_index].pop_back() {
                self.crates[move_val.to_index].push_back(cr);
            }
        }
    }

    pub fn move_crates_9001(&mut self, move_line: &str) {
        let move_val = get_move_values(move_line);

        let mut tmp: VecDeque<char> = VecDeque::new();
        for _ in 0..move_val.quantity {
            if let Some(cr) = self.crates[move_val.from_index].pop_back() {
                tmp.push_front(cr)
            }
        }
        self.crates[move_val.to_index].append(&mut tmp);
    }
}

#[cfg(test)]
mod cargo_tests {
    use super::get_move_values;
    use super::Cargo;
    use indoc::indoc;
    use std::collections::VecDeque;

    #[test]
    fn from_string_tests1() {
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
    fn from_string_tests2() {
        let cargo_input = indoc! {"
            [G]         [P]         [M]    
            [V]     [M] [W] [S]     [Q]    
            [N]     [N] [G] [H]     [T] [F]
            [J]     [W] [V] [Q] [W] [F] [P]
        [C] [H]     [T] [T] [G] [B] [Z] [B]
        [S] [W] [S] [L] [F] [B] [P] [C] [H]
        [G] [M] [Q] [S] [Z] [T] [J] [D] [S]
        [B] [T] [M] [B] [J] [C] [T] [G] [N]
        1   2   3   4   5   6   7   8   9 
        "};
        let cargo = Cargo::from_string(cargo_input);
        let crates = &cargo.crates;
        assert_eq!(crates.len(), 9);
        assert_eq!(crates[0], VecDeque::from(vec!['B', 'G', 'S', 'C']));
        assert_eq!(
            crates[1],
            VecDeque::from(vec!['T', 'M', 'W', 'H', 'J', 'N', 'V', 'G'])
        );
        assert_eq!(crates[2], VecDeque::from(vec!['M', 'Q', 'S']));
        assert_eq!(
            crates[3],
            VecDeque::from(vec!['B', 'S', 'L', 'T', 'W', 'N', 'M'])
        );
        assert_eq!(
            crates[4],
            VecDeque::from(vec!['J', 'Z', 'F', 'T', 'V', 'G', 'W', 'P'])
        );
        assert_eq!(
            crates[5],
            VecDeque::from(vec!['C', 'T', 'B', 'G', 'Q', 'H', 'S'])
        );
        assert_eq!(crates[6], VecDeque::from(vec!['T', 'J', 'P', 'B', 'W']));
        assert_eq!(
            crates[7],
            VecDeque::from(vec!['G', 'D', 'C', 'Z', 'F', 'T', 'Q', 'M'])
        );
        assert_eq!(
            crates[8],
            VecDeque::from(vec!['N', 'S', 'H', 'B', 'P', 'F'])
        );
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
        cargo.crates.push_back(VecDeque::from(vec!['A']));

        assert_eq!(cargo.get_top_crates(), "NDA");
    }

    #[test]
    fn move_crates_9000_test() {
        let mut cargo = Cargo {
            crates: VecDeque::new(),
        };
        cargo.crates.push_back(VecDeque::from(vec!['Z', 'N']));
        cargo.crates.push_back(VecDeque::from(vec!['M', 'C', 'D']));
        cargo.crates.push_back(VecDeque::from(vec!['P']));

        cargo.move_crates_9000("move 1 from 2 to 1");
        {
            let crates = &cargo.crates;
            assert_eq!(crates[0], VecDeque::from(vec!['Z', 'N', 'D']));
            assert_eq!(crates[1], VecDeque::from(vec!['M', 'C']));
            assert_eq!(crates[2], VecDeque::from(vec!['P']));
        }

        cargo.move_crates_9000("move 3 from 1 to 3");
        {
            let crates = &cargo.crates;
            assert_eq!(crates[0], VecDeque::from(vec![]));
            assert_eq!(crates[1], VecDeque::from(vec!['M', 'C']));
            assert_eq!(crates[2], VecDeque::from(vec!['P', 'D', 'N', 'Z']));
        }

        cargo.move_crates_9000("move 2 from 2 to 1");
        {
            let crates = &cargo.crates;
            assert_eq!(crates[0], VecDeque::from(vec!['C', 'M']));
            assert_eq!(crates[1], VecDeque::from(vec![]));
            assert_eq!(crates[2], VecDeque::from(vec!['P', 'D', 'N', 'Z']));
        }

        cargo.move_crates_9000("move 1 from 1 to 2");
        {
            let crates = &cargo.crates;
            assert_eq!(crates[0], VecDeque::from(vec!['C']));
            assert_eq!(crates[1], VecDeque::from(vec!['M']));
            assert_eq!(crates[2], VecDeque::from(vec!['P', 'D', 'N', 'Z']));
        }
    }

    #[test]
    fn move_crates_9001_test() {
        let mut cargo = Cargo {
            crates: VecDeque::new(),
        };
        cargo.crates.push_back(VecDeque::from(vec!['Z', 'N']));
        cargo.crates.push_back(VecDeque::from(vec!['M', 'C', 'D']));
        cargo.crates.push_back(VecDeque::from(vec!['P']));

        cargo.move_crates_9001("move 1 from 2 to 1");
        {
            let crates = &cargo.crates;
            assert_eq!(crates[0], VecDeque::from(vec!['Z', 'N', 'D']));
            assert_eq!(crates[1], VecDeque::from(vec!['M', 'C']));
            assert_eq!(crates[2], VecDeque::from(vec!['P']));
        }

        cargo.move_crates_9001("move 3 from 1 to 3");
        {
            let crates = &cargo.crates;
            assert_eq!(crates[0], VecDeque::from(vec![]));
            assert_eq!(crates[1], VecDeque::from(vec!['M', 'C']));
            assert_eq!(crates[2], VecDeque::from(vec!['P', 'Z', 'N', 'D']));
        }

        cargo.move_crates_9001("move 2 from 2 to 1");
        {
            let crates = &cargo.crates;
            assert_eq!(crates[0], VecDeque::from(vec!['M', 'C']));
            assert_eq!(crates[1], VecDeque::from(vec![]));
            assert_eq!(crates[2], VecDeque::from(vec!['P', 'Z', 'N', 'D']));
        }

        cargo.move_crates_9001("move 1 from 1 to 2");
        {
            let crates = &cargo.crates;
            assert_eq!(crates[0], VecDeque::from(vec!['M']));
            assert_eq!(crates[1], VecDeque::from(vec!['C']));
            assert_eq!(crates[2], VecDeque::from(vec!['P', 'Z', 'N', 'D']));
        }
    }
}
