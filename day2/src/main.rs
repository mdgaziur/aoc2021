mod part1;
mod part2;

use crate::part1::Part1;
use crate::part2::Part2;
use libaoc21::{get_input_for_day_and_part, get_part, AOCDayPart};

fn main() {
    let part = get_part();
    let input = get_input_for_day_and_part(2, part);

    match part {
        1 => Part1::new(&input).solve(),
        2 => Part2::new(&input).solve(),
        _ => panic!("Unknown part!"),
    }
}
