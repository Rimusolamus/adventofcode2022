use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub(crate) fn day_four() -> io::Result<()> {
    let file = File::open("day4.input")?;
    let reader = BufReader::new(file);

    let mut lines_with_dup = 0;

    for line in reader.lines() {
        let line = line?;
        let divided_line = line.split(",").collect::<Vec<&str>>();
        let divided_left = divided_line[0].split("-").collect::<Vec<&str>>();
        let full_left = (divided_left[0].parse::<i32>().unwrap()..divided_left[1].parse::<i32>().unwrap() + 1).collect::<Vec<i32>>();
        let divided_right = divided_line[1].split("-").collect::<Vec<&str>>();
        let full_right = (divided_right[0].parse::<i32>().unwrap()..divided_right[1].parse::<i32>().unwrap() + 1).collect::<Vec<i32>>();
        // probably a better way to do this is to just make a vector of all the numbers and then check if there are any duplicates
        // but I don't care enough to do that
        // also right part is useless probably, but  I have a  feeling that I need it for some cases
        if full_left.iter().any(|&x| full_right.contains(&x)) || full_right.iter().any(|&x| full_left.contains(&x)) {
            lines_with_dup += 1;
        }
    }

    println!("{}", lines_with_dup);

    Ok(())
}