mod day6;
mod day5;
mod day4;
mod day3;
mod day2;
mod day1;
mod day8;

use std::io::{self};

fn main() -> io::Result<()> {
    // day1::day_one().expect("day1 failed");
    // day2::day_two().expect("day2 failed");
    // day3::day_three().expect("day3 failed");
    // day4::day_four().expect("day4 failed");
    // day5::day_five().expect("day5 failed");
    // day6::day_six().expect("day6 failed");
    day8::day_eight().expect("day8 failed");
    println!("Have a nice day!");
    Ok(())
}
