use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub(crate) fn day_three() -> io::Result<()> {
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