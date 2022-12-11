use std::collections::{HashMap, VecDeque};

#[derive(Clone)]
struct RopeGridPoint {
    pub visited_by_tail: bool,
}

impl RopeGridPoint {
    pub fn new() -> RopeGridPoint {
        RopeGridPoint {
            visited_by_tail: false,
        }
    }
}

pub struct RopeGrid {
    grid: HashMap<(i128, i128), RopeGridPoint>,
    pub rope: Vec<(i128, i128)>,
}

impl RopeGrid {
    pub fn new(length_of_rope: usize) -> RopeGrid {
        let mut rgp = RopeGridPoint::new();
        rgp.visited_by_tail = true;
        let mut grid = HashMap::new();
        grid.insert((0, 0), rgp);

        let mut rope = Vec::new();
        rope.resize(length_of_rope, (0, 0));

        RopeGrid { grid, rope }
    }

    fn rope_point_is_not_beside_front_point(&self, point: usize) -> bool {
        !self.rope_point_is_beside_front_point(point)
    }

    fn rope_point_is_beside_front_point(&self, point: usize) -> bool {
        self.rope[point].0 <= self.rope[point - 1].0 + 1
            && self.rope[point].0 >= self.rope[point - 1].0 - 1
            && self.rope[point].1 <= self.rope[point - 1].1 + 1
            && self.rope[point].1 >= self.rope[point - 1].1 - 1
    }

    fn update_grid(&mut self) {
        let tail = self.rope[self.rope.len() - 1];
        let mut tail_p = self
            .grid
            .entry((tail.0, tail.1))
            .or_insert(RopeGridPoint::new());
        tail_p.visited_by_tail = true;
    }

    fn move_rest_of_rope(&mut self) {
        for i in 1..self.rope.len() {
            if self.rope_point_is_not_beside_front_point(i) {
                let front_point = self.rope[i - 1];
                let point = &mut self.rope[i];

                let x_diff = front_point.0 - point.0;
                let y_diff = front_point.1 - point.1;

                if x_diff.abs() == 2 && y_diff.abs() == 2 {
                    point.0 += x_diff / 2;
                    point.1 += y_diff / 2;
                } else if x_diff.abs() == 2 {
                    point.0 += x_diff / 2;
                    point.1 = front_point.1;
                } else if y_diff.abs() == 2 {
                    point.0 = front_point.0;
                    point.1 += y_diff / 2;
                }
            } else {
                break;
            }
        }
    }

    fn move_xy(&mut self, xy: (i128, i128)) {
        let head_p = &mut self.rope[0];
        head_p.0 += xy.0;
        head_p.1 += xy.1;
        self.move_rest_of_rope();
    }

    fn move_right(&mut self) {
        self.move_xy((1, 0));
        self.update_grid();
    }

    fn move_up(&mut self) {
        self.move_xy((0, 1));
        self.update_grid();
    }

    fn move_down(&mut self) {
        self.move_xy((0, -1));
        self.update_grid();
    }

    fn move_left(&mut self) {
        self.move_xy((-1, 0));
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
            if rgp.visited_by_tail {
                sum += 1;
            }
        }

        sum
    }

    pub fn get_head_pos(&self) -> (i128, i128) {
        self.rope[0]
    }

    pub fn get_tail_pos(&self) -> (i128, i128) {
        self.rope[self.rope.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::RopeGrid;
    use super::RopeGridPoint;
    use indoc::indoc;

    #[test]
    fn move_test1() {
        let mut rg = RopeGrid::new(2);
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
        let mut rg = RopeGrid::new(2);
        rg.make_move("R 1");
        let mut head_pos = rg.get_head_pos();
        let mut tail_pos = rg.get_tail_pos();
        assert_eq!(head_pos.0, 1);
        assert_eq!(head_pos.1, 0);
        assert_eq!(tail_pos.0, 0);
        assert_eq!(tail_pos.1, 0);
        assert_eq!(rg.number_of_visited_points_tail(), 1);

        rg.make_move("L 4");
        head_pos = rg.get_head_pos();
        tail_pos = rg.get_tail_pos();
        assert_eq!(head_pos.0, -3);
        assert_eq!(head_pos.1, 0);
        assert_eq!(tail_pos.0, -2);
        assert_eq!(tail_pos.1, 0);
        assert_eq!(rg.number_of_visited_points_tail(), 3);

        rg.make_move("R 4");
        head_pos = rg.get_head_pos();
        tail_pos = rg.get_tail_pos();
        assert_eq!(head_pos.0, 1);
        assert_eq!(head_pos.1, 0);
        assert_eq!(tail_pos.0, 0);
        assert_eq!(tail_pos.1, 0);

        rg.make_move("U 18");
        head_pos = rg.get_head_pos();
        tail_pos = rg.get_tail_pos();
        assert_eq!(head_pos.0, 1);
        assert_eq!(head_pos.1, 18);
        assert_eq!(tail_pos.0, 1);
        assert_eq!(tail_pos.1, 17);
    }

    #[test]
    fn move_test3() {
        let mut rg = RopeGrid::new(2);
        rg.make_move("R 4");
        let mut head_pos = rg.get_head_pos();
        let mut tail_pos = rg.get_tail_pos();
        assert_eq!(head_pos.0, 4);
        assert_eq!(head_pos.1, 0);
        assert_eq!(tail_pos.0, 3);
        assert_eq!(tail_pos.1, 0);
        assert_eq!(rg.number_of_visited_points_tail(), 4);

        rg.make_move("U 4");
        head_pos = rg.get_head_pos();
        tail_pos = rg.get_tail_pos();
        assert_eq!(head_pos.0, 4);
        assert_eq!(head_pos.1, 4);
        assert_eq!(tail_pos.0, 4);
        assert_eq!(tail_pos.1, 3);
        assert_eq!(rg.number_of_visited_points_tail(), 7);

        rg.make_move("L 3");
        head_pos = rg.get_head_pos();
        tail_pos = rg.get_tail_pos();
        assert_eq!(head_pos.0, 1);
        assert_eq!(head_pos.1, 4);
        assert_eq!(tail_pos.0, 2);
        assert_eq!(tail_pos.1, 4);
        assert_eq!(rg.number_of_visited_points_tail(), 9);

        rg.make_move("D 3");
        head_pos = rg.get_head_pos();
        tail_pos = rg.get_tail_pos();
        assert_eq!(head_pos.0, 1);
        assert_eq!(head_pos.1, 1);
        assert_eq!(tail_pos.0, 1);
        assert_eq!(tail_pos.1, 2);
        assert_eq!(rg.number_of_visited_points_tail(), 11);
    }

    #[test]
    fn move_test4() {
        let mut rg = RopeGrid::new(10);
        let input = indoc! {"R 5
        U 8
        L 8
        D 3
        R 17
        D 10
        L 25
        U 20"};

        for line in input.lines() {
            rg.make_move(line);
        }
        assert_eq!(rg.number_of_visited_points_tail(), 36);
    }

}
