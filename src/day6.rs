use std::fs::File;
use std::io::{self, prelude::*, BufReader};


pub(crate) fn day_six() -> io::Result<()> {
    let file = File::open("day6.input")?;
    let reader = BufReader::new(file);
    // only one line in the file is presented
    for line in reader.lines() {
        let line = line?;
        for i in 0..(&line.len() - 13) {
            if is_unique(&line[i..i + 14]) {
                println!("{} is unique", &line[i..i + 14]);
                println!("{}", i + 14);
                break;
            }
        }
    }
    Ok(())
}

// check if all chars in the string are unique
fn is_unique(s: &str) -> bool {
    let mut chars = s.chars();
    let mut unique = true;
    while unique && chars.clone().count() > 0 {
        let c = chars.next().unwrap();
        unique = !chars.clone().any(|x| x == c);
    }
    unique
}