use std::collections::{HashMap, VecDeque};

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

pub struct RopeGrid {
    grid: HashMap<(i128, i128), RopeGridPoint>,
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
        let mut grid = HashMap::new();
        grid.insert((0, 0), rgp);

        RopeGrid {
            grid,
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
        let mut head_p = self
            .grid
            .entry((self.head_pos_x, self.head_pos_y))
            .or_insert(RopeGridPoint::new());
        head_p.visitedByHead = true;
        let mut tail_p = self
            .grid
            .entry((self.tail_pos_x, self.tail_pos_y))
            .or_insert(RopeGridPoint::new());
        tail_p.visitedByTail = true;
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

        if self.tail_is_not_beside_head() {
            self.tail_pos_x = self.head_pos_x;
            self.tail_pos_y = self.head_pos_y + 1;
        }

        self.update_grid();
    }

    fn move_left(&mut self) {
        self.head_pos_x -= 1;

        if self.tail_is_not_beside_head() {
            self.tail_pos_x = self.head_pos_x + 1;
            self.tail_pos_y = self.head_pos_y;
        }

        self.update_grid();
    }

    pub fn make_move(&mut self, mo: &str) {
        let mo_vec: Vec<String> = mo.split(" ").into_iter().map(|s| s.to_string()).collect();
        let direction = &mo_vec[0];
        let steps: i128 = mo_vec[1].parse::<i128>().unwrap();

        for _ in 0..steps {
            if direction == "R" {
                self.move_right();
            } else if direction == "L" {
                self.move_left();
            } else if direction == "D" {
                self.move_down();
            } else if direction == "U" {
                self.move_up();
            }
        }
    }

    pub fn number_of_visited_points_tail(&self) -> i128 {
        let mut sum = 0;

        for (_, rgp) in &self.grid {
            if rgp.visitedByTail {
                sum += 1;
            }
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::RopeGrid;
    use super::RopeGridPoint;
    use indoc::indoc;

    #[test]
    fn move_test1() {
        let mut rg = RopeGrid::new();
        let input = indoc! {"R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2"};

        for line in input.lines() {
            rg.make_move(line);
        }
        assert_eq!(rg.number_of_visited_points_tail(), 13);
    }

    #[test]
    fn move_test2() {
        let mut rg = RopeGrid::new();
        rg.make_move("R 1");
        assert_eq!(rg.head_pos_x, 1);
        assert_eq!(rg.head_pos_y, 0);
        assert_eq!(rg.tail_pos_x, 0);
        assert_eq!(rg.tail_pos_y, 0);
        assert_eq!(rg.number_of_visited_points_tail(), 1);

        rg.make_move("L 4");
        assert_eq!(rg.head_pos_x, -3);
        assert_eq!(rg.head_pos_y, 0);
        assert_eq!(rg.tail_pos_x, -2);
        assert_eq!(rg.tail_pos_y, 0);
        assert_eq!(rg.number_of_visited_points_tail(), 3);

        rg.make_move("R 4");
        assert_eq!(rg.head_pos_x, 1);
        assert_eq!(rg.head_pos_y, 0);
        assert_eq!(rg.tail_pos_x, 0);
        assert_eq!(rg.tail_pos_y, 0);

        rg.make_move("U 18");
        assert_eq!(rg.head_pos_x, 1);
        assert_eq!(rg.head_pos_y, 18);
        assert_eq!(rg.tail_pos_x, 1);
        assert_eq!(rg.tail_pos_y, 17);
    }

    #[test]
    fn move_test3() {
        let mut rg = RopeGrid::new();
        rg.make_move("R 4");
        assert_eq!(rg.head_pos_x, 4);
        assert_eq!(rg.head_pos_y, 0);
        assert_eq!(rg.tail_pos_x, 3);
        assert_eq!(rg.tail_pos_y, 0);
        assert_eq!(rg.number_of_visited_points_tail(), 4);

        rg.make_move("U 4");
        assert_eq!(rg.head_pos_x, 4);
        assert_eq!(rg.head_pos_y, 4);
        assert_eq!(rg.tail_pos_x, 4);
        assert_eq!(rg.tail_pos_y, 3);
        assert_eq!(rg.number_of_visited_points_tail(), 7);

        rg.make_move("L 3");
        assert_eq!(rg.head_pos_x, 1);
        assert_eq!(rg.head_pos_y, 4);
        assert_eq!(rg.tail_pos_x, 2);
        assert_eq!(rg.tail_pos_y, 4);
        assert_eq!(rg.number_of_visited_points_tail(), 9);

        rg.make_move("D 3");
        assert_eq!(rg.head_pos_x, 1);
        assert_eq!(rg.head_pos_y, 1);
        assert_eq!(rg.tail_pos_x, 1);
        assert_eq!(rg.tail_pos_y, 2);
        assert_eq!(rg.number_of_visited_points_tail(), 11);
    }
}
