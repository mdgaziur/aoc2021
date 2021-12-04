use libaoc21::fake_stdin::FakeStdin;
use libaoc21::AOCDayPart;
use std::io::Read;

pub struct Part2 {
    stdin: FakeStdin,
}

impl Part2 {
    pub fn new(input: &str) -> Self {
        Part2 {
            stdin: FakeStdin::new(input.as_bytes()),
        }
    }
}

impl AOCDayPart for Part2 {
    fn solve(&mut self) {
        let mut input = String::new();
        let mut nums = Vec::new();
        let mut increased = 0;

        while let Ok(_) = self.stdin.read_to_string(&mut input) {
            nums.push(input.parse::<i32>().expect("Invalid input"));
            input = String::new();
        }

        for idx in 0..nums.len() - 3 {
            let sum1 = nums[idx] + nums[idx + 1] + nums[idx + 2];
            let sum2 = nums[idx + 1] + nums[idx + 2] + nums[idx + 3];

            increased += i32::from(sum2 > sum1);
        }

        println!("{}", increased);
    }
}
