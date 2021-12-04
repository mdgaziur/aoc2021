use std::io::Read;
use libaoc21::AOCDayPart;
use libaoc21::fake_stdin::FakeStdin;

pub struct Part2 {
    stdin: FakeStdin
}

impl Part2 {
    pub fn new(input: &str) -> Self {
        Self {
            stdin: FakeStdin::new(input.as_bytes())
        }
    }

    fn solve_o2_gen(&self, inputs: Vec<String>, idx: usize) -> Vec<String> {
        if inputs.len() == 1 {
            return inputs;
        }
        let mut zero_count = 0;
        let mut one_count = 0;

        for input in &inputs {
            if input.chars().nth(idx).unwrap() == '1' {
                one_count += 1;
            }
            else {
                zero_count += 1;
            }
        }

        let mut temp = Vec::new();
        let match_against = if one_count > zero_count || one_count == zero_count { '1' } else { '0' };
        for input in &inputs {
            if input.chars().nth(idx).unwrap() == match_against {
                temp.push(input.clone());
            }
        }

        self.solve_o2_gen(temp, idx + 1)
    }

    fn solve_co2_scrub(&self, inputs: Vec<String>, idx: usize) -> Vec<String> {
        if inputs.len() <= 1 {
            return inputs;
        }
        let mut zero_count = 0;
        let mut one_count = 0;

        for input in &inputs {
            if input.chars().nth(idx).unwrap() == '1' {
                one_count += 1;
            }
            else {
                zero_count += 1;
            }
        }

        let mut temp = Vec::new();
        let match_against = if zero_count < one_count || zero_count == one_count { '0' } else { '1' };
        for input in &inputs {
            if input.chars().nth(idx).unwrap() == match_against {
                temp.push(input.clone());
            }
        }

        self.solve_co2_scrub(temp, idx + 1)
    }
}

impl AOCDayPart for Part2 {
    fn solve(&mut self) {
        let mut input = String::new();
        let mut inputs = Vec::new();

        while let Ok(_) = self.stdin.read_to_string(&mut input) {
            inputs.push(input);
            input = String::new();
        }

        let o2 = self.solve_o2_gen(inputs.clone(), 0);
        let co2 = self.solve_co2_scrub(inputs.clone(), 0);

        let o2_rating = i32::from_str_radix(&o2[0], 2).unwrap();
        let co2_rating = i32::from_str_radix(&co2[0], 2).unwrap();

        println!("{}", o2_rating * co2_rating);
    }
}