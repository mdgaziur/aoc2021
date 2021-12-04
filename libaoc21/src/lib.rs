pub mod fake_stdin;

pub fn get_part() -> u8 {
    let mut args = std::env::args();
    args.next().unwrap();

    let day = args.next().expect("Expected a number indicating part");

    day.parse().expect("Expected number, got invalid type")
}

/// Tries to read input.txt for given day and part and puts it
/// in STDIN
pub fn get_input_for_day_and_part(day: u8, part: u8) -> String {
    std::fs::read_to_string(format!("testcases/day{}-part{}.txt", day, part)).expect(&format!(
        "Cannot read input for day: {}, part: {}",
        day, part
    ))
}

pub trait AOCDayPart {
    fn solve(&mut self);
}
