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
        let mut previous_depth = -1;
        let mut increased = 0;

        while let Ok(_) = self.stdin.read_to_string(&mut input) {
            let depth = input.parse::<i16>().expect("Invalid input");

            if previous_depth < depth && previous_depth != -1 {
                increased += 1;
            }

            previous_depth = depth;
            input = String::new();
        }

        println!("{}", increased);
    }
}
