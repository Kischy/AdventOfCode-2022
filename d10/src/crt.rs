use std::collections::{HashMap, VecDeque};

pub struct CRT {
    pub signal_strengths: HashMap<i128, i128>,
    current_cycle: i128,
    register_x: i128,
    screen: VecDeque<VecDeque<char>>,
}

impl CRT {
    pub fn new() -> CRT {
        let mut screen = VecDeque::new();
        screen.resize(6, VecDeque::new());

        for row in &mut screen {
            row.resize(40, '.');
        }

        CRT {
            signal_strengths: HashMap::new(),
            current_cycle: 0,
            register_x: 1,
            screen: screen,
        }
    }

    pub fn draw_screen_and_increase_cycle(&mut self) {
        let row_index = (self.current_cycle / 40) as usize;
        let num_of_rows = self.screen.len();
        if row_index < num_of_rows {
            let drawing_pos = self.current_cycle % 40;
            print!("{},", drawing_pos);
            if drawing_pos <= self.register_x + 1 && drawing_pos >= self.register_x - 1 {
                self.screen[row_index][drawing_pos as usize] = '#';
            }
        }

        self.current_cycle += 1;
        if self.current_cycle % 20 == 0 {
            self.signal_strengths
                .insert(self.current_cycle, self.register_x * self.current_cycle);
        }
    }

    pub fn calc_instruction(&mut self, instruction: &str) {
        if instruction == "noop" {
            self.draw_screen_and_increase_cycle()
        } else {
            let inst_vec: Vec<String> = instruction
                .split(" ")
                .into_iter()
                .map(|s| s.to_string())
                .collect();
            let inst = &inst_vec[0];
            let number: i128 = inst_vec[1].parse::<i128>().unwrap();

            if inst == "addx" {
                self.draw_screen_and_increase_cycle();
                self.draw_screen_and_increase_cycle();
                self.register_x += number;
            }
        }
    }

    pub fn print_screen(&self) {
        for row in &self.screen {
            for letter in row {
                print!("{}", letter);
            }
            print!("\n");
        }
    }
}
