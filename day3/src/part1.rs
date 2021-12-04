use libaoc21::fake_stdin::FakeStdin;
use libaoc21::AOCDayPart;
use std::io::Read;

pub struct Part1 {
    stdin: FakeStdin,
}

impl Part1 {
    pub fn new(input: &str) -> Self {
        Self {
            stdin: FakeStdin::new(input.as_bytes()),
        }
    }
}

impl AOCDayPart for Part1 {
    fn solve(&mut self) {
        let mut input = String::new();
        let mut zero_count: Vec<usize> = vec![0; 12];
        let mut one_count: Vec<usize> = vec![0; 12];

        while let Ok(_) = self.stdin.read_to_string(&mut input) {
            for idx in 0..input.len() {
                if input.chars().nth(idx).unwrap() == '0' {
                    zero_count[idx] += 1;
                } else {
                    one_count[idx] += 1;
                }
            }

            input = String::new();
        }

        let mut gamma_bits = String::new();
        let mut epsilon_bits = String::new();
        for idx in 0..12 {
            if zero_count[idx] > one_count[idx] {
                gamma_bits += "0";
                epsilon_bits += "1";
            } else {
                gamma_bits += "1";
                epsilon_bits += "0";
            }
        }

        let gamma = i32::from_str_radix(&gamma_bits, 2).unwrap();
        let epsilon_rate = i32::from_str_radix(&epsilon_bits, 2).unwrap();
        println!("{}", gamma * epsilon_rate);
    }
}
