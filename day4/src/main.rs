mod part1;

use crate::part1::Part1;
use libaoc21::{get_input_for_day_and_part, get_part, AOCDayPart};

fn main() {
    let part = get_part();
    let input = get_input_for_day_and_part(4, part);

    match part {
        1 => Part1::new(&input).solve(),
        2 => unimplemented!(),
        _ => panic!("Unknown part!"),
    }
}
