use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    // day_one()
    // day_two();
    day_three();

    println!("Have a nice day!");
    Ok(())
}

fn day_three() -> io::Result<()> {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();

    let file = File::open("day3.input")?;
    let reader = BufReader::new(file);

    let mut current_sum = 0;
    let lines_as_vec: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    for (i, line) in lines_as_vec.iter().enumerate() {
        if i == 0 || i % 3 == 0 {
            // I HAVE NO IDEA WHY I NEED THIS IF STATEMENT
            if i + 1 < lines_as_vec.len() {
                let line2: Vec<char> = lines_as_vec[i + 1].chars().collect();
                let line3: Vec<char> = lines_as_vec[i + 2].chars().collect();

                // iterate over the left side
                for char in line.chars() {
                    // if the character is in the right side
                    if line2.contains(&char) && line3.contains(&char) {
                        let index = alphabet.iter().position(|&c| c == char).unwrap();
                        current_sum += index + 1;
                        break;
                    }
                }
            }
        }
    }
    println!("{}", current_sum);

    Ok(())
}

fn day_one() -> io::Result<()> {
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
    let last_three = &numbers[numbers.len() - 3..numbers.len()];

    println!("Best sum: {}", last_three[0] + last_three[1] + last_three[2]);

    Ok(())
}

fn day_two() -> io::Result<()> {
    let file = File::open("day2.input")?;
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