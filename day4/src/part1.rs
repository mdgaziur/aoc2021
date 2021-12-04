use libaoc21::fake_stdin::FakeStdin;
use libaoc21::AOCDayPart;
use std::io::Read;

#[derive(Debug)]
struct Number {
    value: u8,
    marked: bool,
}

#[derive(Debug)]
struct Row {
    values: Vec<Number>,
    complete: bool,
}

#[derive(Debug)]
struct Board {
    rows: Vec<Row>,
}

impl Board {
    pub fn unmarked_rows_total(&self) -> u16 {
        let mut sum: u16 = 0;
        for row in &self.rows {
            if row.complete {
                continue;
            }

            for val in &row.values {
                if val.marked {
                    continue;
                }
                sum += val.value as u16;
            }
        }

        sum
    }
}

pub struct Part1 {
    stdin: FakeStdin,
}

impl Part1 {
    pub fn new(input: &str) -> Self {
        Self {
            stdin: FakeStdin::new(input.as_bytes()),
        }
    }

    /// Draws once and returns a board if it has one row complete
    fn draw_once(num: u8, boards: &mut [Board]) -> Option<&Board> {
        for board in boards {
            for row in &mut board.rows {
                let mut marked = 0;
                for val in &mut row.values {
                    if val.value == num {
                        marked += 1;
                        val.marked = true;
                    }
                    else if val.marked {
                        marked += 1;
                    }
                }

                if marked == 5 {
                    row.complete = true;
                    return Some(board);
                }
            }
        }

        None
    }
}

impl AOCDayPart for Part1 {
    fn solve(&mut self) {
        let mut input = String::new();
        self.stdin.read_to_string(&mut input).unwrap();

        let draws = input
            .split(',')
            .map(|d| d.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        let mut boards: Vec<Board> = Vec::new();

        loop {
            input = String::new();
            if let Ok(_) = self.stdin.read_to_string(&mut input) {
                let mut board: Board = Board { rows: vec![] };

                board.rows.push(Row {
                    values: input
                        .split_ascii_whitespace()
                        .map(|n| Number{ value: n.parse::<u8>().unwrap(), marked: false })
                        .collect(),
                    complete: false,
                });
                input = String::new();

                for _ in 0..4 {
                    self.stdin.read_to_string(&mut input).unwrap();
                    board.rows.push(Row {
                        values: input
                            .split_ascii_whitespace()
                            .map(|n| Number{ value: n.parse::<u8>().unwrap(), marked: false })
                            .collect(),
                        complete: false,
                    });
                    input = String::new();
                }

                boards.push(board);
            } else {
                break;
            }
        }

        for num in draws {
            if let Some(board) = Part1::draw_once(num, &mut boards) {
                println!("{}", num as u32 * (board.unmarked_rows_total() as u32));
                break;
            }
        }
    }
}
