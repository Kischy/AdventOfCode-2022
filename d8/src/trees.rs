use std::collections::VecDeque;

#[derive(Debug)]
pub struct Tree {
    pub is_visible: bool,
    pub heigth: i32,
    scenic_view: u128,
}

impl Tree {
    pub fn new(heigth: i32) -> Tree {
        Tree {
            is_visible: false,
            heigth: heigth,
            scenic_view: 0,
        }
    }
}

pub struct Trees {
    pub trees: VecDeque<VecDeque<Tree>>,
}

impl Trees {
    fn calculate_visibility(&mut self) {
        for tree_row in self.trees.iter_mut() {
            let mut highest = -1;
            for tree in tree_row {
                if tree.heigth > highest {
                    tree.is_visible = true;
                }
                if tree.heigth > highest {
                    highest = tree.heigth;
                }
            }
        }

        for tree_row in self.trees.iter_mut() {
            let mut highest = -1;
            for tree in tree_row.iter_mut().rev() {
                if tree.heigth > highest {
                    tree.is_visible = true;
                }
                if tree.heigth > highest {
                    highest = tree.heigth;
                }
            }
        }

        let inner_len = self.trees[0].len();

        for i in 0..inner_len {
            let mut highest = -1;
            for j in 0..self.trees.len() {
                let mut tree = &mut self.trees[j][i];
                if tree.heigth > highest {
                    tree.is_visible = true;
                }
                if tree.heigth > highest {
                    highest = tree.heigth;
                }
            }
        }

        for i in 0..inner_len {
            let mut highest = -1;
            for j in (0..self.trees.len()).rev() {
                let mut tree = &mut self.trees[j][i];
                if tree.heigth > highest {
                    tree.is_visible = true;
                }
                if tree.heigth > highest {
                    highest = tree.heigth;
                }
            }
        }
    }

    fn calculate_scenic_val(&mut self, out_ind: usize, inner_ind: usize) {
        let outer_len = self.trees.len();
        let inner_len = self.trees[0].len();
        let mut right = 0;
        let mut left = 0;
        let mut top = 0;
        let mut down = 0;

        let tree_heigth = self.trees[out_ind][inner_ind].heigth;

        for j in inner_ind + 1..inner_len {
            right += 1;
            if self.trees[out_ind][j].heigth >= tree_heigth {
                break;
            }
        }

        for j in (0..inner_ind).rev() {
            left += 1;
            if self.trees[out_ind][j].heigth >= tree_heigth {
                break;
            }
        }

        for j in out_ind + 1..outer_len {
            down += 1;
            if self.trees[j][inner_ind].heigth >= tree_heigth {
                break;
            }
        }

        for j in (0..out_ind).rev() {
            top += 1;
            if self.trees[j][inner_ind].heigth >= tree_heigth {
                break;
            }
        }

        self.trees[out_ind][inner_ind].scenic_view = right * left * top * down;
    }

    fn calculate_scenic_view(&mut self) {
        let outer_len = self.trees.len();
        let inner_len = self.trees[0].len();

        for i in 1..outer_len - 1 {
            for j in 1..inner_len - 1 {
                self.calculate_scenic_val(i, j)
            }
        }
    }

    pub fn from_string(input: &str) -> Trees {
        let mut trees: VecDeque<VecDeque<Tree>> = VecDeque::new();
        for line in input.lines() {
            trees.push_back(VecDeque::new());
            for num in line.chars() {
                if let Some(digit) = num.to_digit(10) {
                    trees
                        .back_mut()
                        .unwrap()
                        .push_back(Tree::new(digit.to_string().parse::<i32>().unwrap()));
                }
            }
        }

        let mut trs = Trees { trees: trees };
        trs.calculate_visibility();
        trs.calculate_scenic_view();

        trs
    }

    pub fn number_of_visible(&self) -> u128 {
        let mut visible = 0;
        for tree_row in &self.trees {
            for tree in tree_row {
                if tree.is_visible {
                    visible += 1;
                }
            }
        }
        visible
    }

    pub fn highest_scenic_view(&self) -> u128 {
        let mut max = 0;
        for tree_row in &self.trees {
            for tree in tree_row {
                if tree.scenic_view > max {
                    max = tree.scenic_view;
                }
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::Trees;
    use indoc::indoc;

    #[test]
    fn from_string_tests() {
        let trees = Trees::from_string(indoc! {"
        30373
        25512
        65332
        33549
        35390
        "
        });
        assert_eq!(trees.number_of_visible(), 21);
    }
}
