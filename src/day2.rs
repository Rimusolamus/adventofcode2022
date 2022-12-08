use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub(crate) fn day_two() -> io::Result<()> {
    let file = File::open("inputs/day2.input")?;
    let reader = BufReader::new(file);

    let mut result = 0;

    for line in reader.lines() {
        // line as char array
        let line = line.unwrap();
        let first_char = line.chars().next().unwrap();
        let last_char = line.chars().last().unwrap();

        // 1 for Rock, 2 for Paper, 3 for Scissors

        // X for lose, Y for draw, Z for lose
        // for rock
        if first_char == 'A' && last_char == 'X' {
            // 3 + 0
            result += 3;
        }
        if first_char == 'A' && last_char == 'Y' {
            // 1 + 3
            result += 4;
        }
        if first_char == 'A' && last_char == 'Z' {
            // 2 + 6
            result += 8;
        }
        // for paper
        if first_char == 'B' && last_char == 'X' {
            // 1 + 0
            result += 1;
        }
        if first_char == 'B' && last_char == 'Y' {
            // 2 + 3
            result += 5;
        }
        if first_char == 'B' && last_char == 'Z' {
            // 3 + 6
            result += 9;
        }
        // for scissors
        if first_char == 'C' && last_char == 'X' {
            // 2 + 0
            result += 2;
        }
        if first_char == 'C' && last_char == 'Y' {
            // 3 + 3
            result += 6;
        }
        if first_char == 'C' && last_char == 'Z' {
            // 1 + 6
            result += 7;
        }
    }
    println!("{}", result);

    Ok(())
}