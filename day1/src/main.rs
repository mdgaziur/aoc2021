mod part1;
mod part2;

use libaoc21::{AOCDayPart, get_input_for_day_and_part, get_part};
use crate::part1::Part1;
use crate::part2::Part2;

fn main() {
    let part = get_part();
    let input = get_input_for_day_and_part(1, part);

    match part {
        1 => Part1::new(&input).solve(),
        2 => Part2::new(&input).solve(),
        _ => panic!("Unknown part!")
    }
}
