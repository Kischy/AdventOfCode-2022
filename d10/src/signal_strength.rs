use std::collections::HashMap;

pub struct SignalStrengthCalculator {
    pub signal_strengths: HashMap<i128, i128>,
    current_cycle: i128,
    register_X: i128,
}

impl SignalStrengthCalculator {
    pub fn new() -> SignalStrengthCalculator {
        SignalStrengthCalculator {
            signal_strengths: HashMap::new(),
            current_cycle: 0,
            register_X: 1,
        }
    }

    pub fn increase_cycle(&mut self) {
        self.current_cycle += 1;
        if self.current_cycle % 20 == 0 {
            self.signal_strengths
                .insert(self.current_cycle, self.register_X * self.current_cycle);
        }
    }

    pub fn calc_instruction(&mut self, instruction: &str) {
        if instruction == "noop" {
            self.increase_cycle()
        } else {
            let inst_vec: Vec<String> = instruction
                .split(" ")
                .into_iter()
                .map(|s| s.to_string())
                .collect();
            let inst = &inst_vec[0];
            let number: i128 = inst_vec[1].parse::<i128>().unwrap();

            if inst == "addx" {
                self.increase_cycle();
                self.increase_cycle();
                self.register_X += number;
            }
        }
    }
}
