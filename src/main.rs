use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut current_sum = 0;

    // list of numbers
    let mut numbers: Vec<i32> = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            if line == "" {
                numbers.push(current_sum);
                current_sum = 0;
            } else {
                current_sum += line.parse::<i32>().unwrap();
            }
        }
    }
    numbers.sort();

    // get three last numbers
    let last_three = &numbers[numbers.len()-3..numbers.len()];

    println!("Best sum: {}", last_three[0] + last_three[1] + last_three[2]);

    Ok(())
}