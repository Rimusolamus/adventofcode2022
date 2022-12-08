use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub(crate) fn day_five() -> io::Result<()> {
    /*      [M]     [B]             [N]
    [T]     [H]     [V] [Q]         [H]
    [Q]     [N]     [H] [W] [T]     [Q]
    [V]     [P] [F] [Q] [P] [C]     [R]
    [C]     [D] [T] [N] [N] [L] [S] [J]
    [D] [V] [W] [R] [M] [G] [R] [N] [D]
    [S] [F] [Q] [Q] [F] [F] [F] [Z] [S]
    [N] [M] [F] [D] [R] [C] [W] [T] [M]
     1   2   3   4   5   6   7   8   9

     1 param - amount
     2 param - from
     3 param - to */

    let file = File::open("inputs/day5.input")?;
    let reader = BufReader::new(file);

    let mut base: Vec<Vec<&str>> = Vec::new();
    base.push(Vec::from(["N", "S", "D", "C", "V", "Q", "T"]));
    base.push(Vec::from(["M", "F", "V"]));
    base.push(Vec::from(["F", "Q", "W", "D", "P", "N", "H", "M"]));
    base.push(Vec::from(["D", "Q", "R", "T", "F"]));
    base.push(Vec::from(["R", "F", "M", "N", "Q", "H", "V", "B"]));
    base.push(Vec::from(["C", "F", "G", "N", "P", "W", "Q"]));
    base.push(Vec::from(["W", "F", "R", "L", "C", "T"]));
    base.push(Vec::from(["T", "Z", "N", "S"]));
    base.push(Vec::from(["M", "S", "D", "J", "R", "Q", "H", "N"]));

    for line in reader.lines() {
        let line = line?;
        let split_line = line.split(",").collect::<Vec<&str>>();
        let from_crate = base[(split_line[1].parse::<i32>().unwrap() - 1) as usize]
            .as_slice()[base[(split_line[1].parse::<i32>().unwrap() - 1) as usize].len() - (split_line[0].parse::<i32>().unwrap() as usize)..]
            .to_vec();
        let final_len = base[(split_line[1].parse::<i32>().unwrap() - 1) as usize].len().saturating_sub(split_line[0].parse::<i32>().unwrap() as usize);
        base[(split_line[1].parse::<i32>().unwrap() - 1) as usize].truncate(final_len);
        base[(split_line[2].parse::<i32>().unwrap() - 1) as usize].append(&mut from_crate.clone());
    }

    for i in 0..base.len() {
        print!("{}", base[i].last().unwrap());
    }

    Ok(())
}