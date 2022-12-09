use std::collections::VecDeque;

#[derive(Clone)]
struct RopeGridPoint {
    pub visitedByHead: bool,
    pub visitedByTail: bool,
}

impl RopeGridPoint {
    pub fn new() -> RopeGridPoint {
        RopeGridPoint {
            visitedByHead: false,
            visitedByTail: false,
        }
    }
}

struct RopeGrid {
    grid: VecDeque<VecDeque<RopeGridPoint>>,
    pub head_pos_x: i128,
    pub head_pos_y: i128,
    pub tail_pos_x: i128,
    pub tail_pos_y: i128,
}

impl RopeGrid {
    pub fn new() -> RopeGrid {
        let mut rgp = RopeGridPoint::new();
        rgp.visitedByHead = true;
        rgp.visitedByTail = true;
        let mut grid = VecDeque::new();
        grid.push_back(VecDeque::new());
        grid[0].push_back(rgp);

        RopeGrid {
            grid: grid,
            head_pos_x: 0,
            head_pos_y: 0,
            tail_pos_x: 0,
            tail_pos_y: 0,
        }
    }
    fn tail_is_not_beside_head(&self) -> bool {
        !self.tail_is_beside_head()
    }

    fn tail_is_beside_head(&self) -> bool {
        self.tail_pos_x <= self.head_pos_x + 1
            && self.tail_pos_x >= self.head_pos_x - 1
            && self.tail_pos_y <= self.head_pos_y + 1
            && self.tail_pos_y >= self.head_pos_y - 1
    }

    fn update_grid(&mut self) {
        let resize_y = if self.head_pos_y > self.tail_pos_y {
            self.head_pos_y + 1
        } else {
            self.tail_pos_y + 1
        };

        self.grid.resize(resize_y as usize, VecDeque::new());

        let resize_x = if self.head_pos_x > self.tail_pos_x {
            self.head_pos_x + 1
        } else {
            self.tail_pos_x + 1
        };

        for row in &mut self.grid {
            row.resize(resize_x as usize, RopeGridPoint::new());
        }

        let head_pos = &mut self.grid[self.head_pos_y as usize][self.head_pos_x as usize];
        head_pos.visitedByHead = true;
        let head_pos = &mut self.grid[self.tail_pos_y as usize][self.tail_pos_x as usize];
        head_pos.visitedByTail = true;
    }

    fn move_right(&mut self) {
        self.head_pos_x += 1;

        if self.tail_is_not_beside_head() {
            self.tail_pos_x = self.head_pos_x - 1;
            self.tail_pos_y = self.head_pos_y;
        }

        self.update_grid();
    }

    fn move_up(&mut self) {
        self.head_pos_y += 1;

        if self.tail_is_not_beside_head() {
            self.tail_pos_x = self.head_pos_x;
            self.tail_pos_y = self.head_pos_y - 1;
        }

        self.update_grid();
    }

    fn move_down(&mut self) {
        self.head_pos_y -= 1;

        if self.head_pos_y < 0 {
            self.head_pos_y = 0;
        }

        if self.tail_is_not_beside_head() {
            self.tail_pos_x = self.head_pos_x;
            self.tail_pos_y = self.head_pos_y + 1;
        }

        self.update_grid();
    }

    fn move_left(&mut self) {
        self.head_pos_x -= 1;

        if self.head_pos_x < 0 {
            self.head_pos_x = 0;
        }

        if self.tail_is_not_beside_head() {
            self.tail_pos_x = self.head_pos_x + 1;
            self.tail_pos_y = self.head_pos_y;
        }

        self.update_grid();
    }

    pub fn make_move(&mut self, mo: &str) {
        let mo_vec: Vec<char> = mo.chars().into_iter().collect();
        let direction = mo_vec[0];
        let steps: i128 = mo_vec[2].to_digit(10).unwrap() as i128;

        for _ in 0..steps {
            if direction == 'R' {
                self.move_right();
            } else if direction == 'L' {
                self.move_left();
            } else if direction == 'D' {
                self.move_down();
            } else if direction == 'U' {
                self.move_up();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RopeGrid;
    use super::RopeGridPoint;
    use indoc::indoc;

    #[test]
    fn from_string_tests() {
        let mut rg = RopeGrid::new();
        rg.make_move("R 1");
        assert_eq!(rg.head_pos_x, 1);
        assert_eq!(rg.head_pos_y, 0);
        assert_eq!(rg.tail_pos_x, 0);
        assert_eq!(rg.tail_pos_y, 0);

        rg.make_move("L 4");
        assert_eq!(rg.head_pos_x, 0);
        assert_eq!(rg.head_pos_y, 0);
        assert_eq!(rg.tail_pos_x, 0);
        assert_eq!(rg.tail_pos_y, 0);

        rg.make_move("R 4");
        assert_eq!(rg.head_pos_x, 4);
        assert_eq!(rg.head_pos_y, 0);
        assert_eq!(rg.tail_pos_x, 3);
        assert_eq!(rg.tail_pos_y, 0);

        rg.make_move("U 8");
        assert_eq!(rg.head_pos_x, 4);
        assert_eq!(rg.head_pos_y, 8);
        assert_eq!(rg.tail_pos_x, 4);
        assert_eq!(rg.tail_pos_y, 7);

        rg.make_move("L 2");
        assert_eq!(rg.head_pos_x, 2);
        assert_eq!(rg.head_pos_y, 8);
        assert_eq!(rg.tail_pos_x, 3);
        assert_eq!(rg.tail_pos_y, 8);

        rg.make_move("D 2");
        assert_eq!(rg.head_pos_x, 2);
        assert_eq!(rg.head_pos_y, 6);
        assert_eq!(rg.tail_pos_x, 2);
        assert_eq!(rg.tail_pos_y, 7);
    }
}
