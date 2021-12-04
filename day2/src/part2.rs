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
        let mut pos: (i32, i32, i32) = (0, 0, 0);

        while let Ok(_) = self.stdin.read_to_string(&mut input) {
            let split = input.split_ascii_whitespace().collect::<Vec<&str>>();
            let cmd = split[0];
            let steps = split[1].parse::<i32>().expect("Invalid input");

            match cmd {
                "forward" => {
                    pos.0 += steps;
                    pos.1 += pos.2 * steps
                }
                "down" => pos.2 += steps,
                "up" => pos.2 -= steps,
                e => panic!("Invalid command: {}", e),
            }

            input = String::new();
        }

        println!("{}", pos.0 * pos.1);
    }
}
