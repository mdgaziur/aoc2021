use std::io::Read;
use libaoc21::AOCDayPart;
use libaoc21::fake_stdin::FakeStdin;

pub struct Part1 {
    stdin: FakeStdin
}

impl Part1 {
    pub fn new(input: &str) -> Self {
        Part1 {
            stdin: FakeStdin::new(input.as_bytes())
        }
    }
}

impl AOCDayPart for Part1 {
    fn solve(&mut self) {
        let mut input = String::new();
        let mut pos: (i32, i32) = (0, 0);

        while let Ok(_) = self.stdin.read_to_string(&mut input) {
            let split = input.split_ascii_whitespace().collect::<Vec<&str>>();
            let cmd = split[0];
            let steps = split[1].parse::<i32>().expect("Invalid input");

            match cmd {
                "forward" => pos.0 += steps,
                "down" => pos.1 += steps,
                "up" => pos.1 -= steps,
                e => panic!("Invalid command: {}", e)
            }

            input = String::new();
        }

        println!("{}", pos.0 * pos.1);
    }
}