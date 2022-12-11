use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub(crate) fn day_eight() -> io::Result<()> {
    let file = File::open("inputs/day8.input")?;
    let reader = BufReader::new(file);
    let mut matrix = Vec::new();
    // fill the matrix
    for line in reader.lines() {
        let line = line?;
        let mut row = Vec::new();
        //push chars into the row as integers
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap());
        }
        matrix.push(row);
    }
    println!("Part1 visible trees: {}", matrix.iter()
        .enumerate()
        .map(|(y, line)| line.iter()
            .enumerate()
            .filter(|(x, _)| look(*x, y, &matrix).0)
            .count())
        .sum::<usize>());

    println!("Part2 scenic score: {}", matrix.iter()
        .enumerate()
        .map(|(y, line)| line.iter()
            .enumerate()
            .map(|(x, _)| look(x, y, &matrix).1)
            .max().expect("Must have tres"))
        .max().expect("Must have tree rows"));
    Ok(())
}

fn look(x: usize, y: usize, map: &Vec<Vec<u32>>) -> (bool, usize) {
    let current_tree = map[y][x];

    let y_max = map.len() as i32;
    let x_max = map[0].len() as i32;
    let ray_max = y_max.max(x_max);

    let dirs: Vec<(i32, i32)> = vec![
        (0, -1), // Up
        (0, 1), // Down
        (-1, 0), // Left
        (1, 0), // Right
    ];

    let mut trees = Vec::new();
    let mut outside_visible = false;
    for dir in dirs {
        // "Shoot" in dir until border
        for length in 1..ray_max {
            let new_x = x as i32 + (dir.0 * length);
            let new_y = y as i32 + (dir.1 * length);
            if new_x < 0 || new_y < 0 || new_x >= x_max || new_y >= y_max {
                let prev_x = (x as i32 + (dir.0 * (length - 1))) as usize;
                let prev_y = (y as i32 + (dir.1 * (length - 1))) as usize;
                let trees_seen_to_border = (x.max(prev_x) - x.min(prev_x)) + (y.max(prev_y) - y.min(prev_y));
                trees.push(trees_seen_to_border);
                outside_visible = true;
                break;
            }

            if map[new_y as usize][new_x as usize] >= current_tree {
                let trees_seen_to_border = (x.max(new_x as usize) - x.min(new_x as usize)) + (y.max(new_y as usize) - y.min(new_y as usize));
                trees.push(trees_seen_to_border);
                break;
            }
        }
    }
    return (outside_visible, trees.iter().fold(1, |acc, v| acc * v));
}
