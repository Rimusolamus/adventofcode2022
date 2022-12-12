use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use ndarray::Array2;

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
    };

    // generate zeroes matrix
    let mut result_matrix = Array2::<u32>::zeros((matrix.len(), matrix.len()));

    let mut max = 0;

    // print result matrix
    for y in 0..matrix.len() {
        for x in 0..matrix.len() {
            let mut result = 1;
            println!("$$$$$$$$$$$$$$$$$$$$$");
            println!("{} &&&&&", matrix[y][x]);
            println!("####################");

            let left = visible_to_left(&matrix, x, y);
            result *= left;
            println!("{:?} - result of left", left);
            let right = visible_to_right(&matrix, x, y);
            result *= right;
            println!("{:?} - result of right", right);
            let top = visible_to_top(&matrix, x, y);
            result *= top;
            println!("{:?} - result of top", top);
            let bottom = visible_to_bottom(&matrix, x, y);
            result *= bottom;
            println!("{:?} - result of bottom", bottom);

            if result > max {
                max = result;
            }

            // if visible_to_left(&matrix, x, y) {
            //     if matrix[y][x] == 0 { result_matrix[[y, x]] = 10 } else {
            //         result_matrix[[y, x]] = matrix[y][x];
            //     }
            // }
            // if visible_to_right(&matrix, x, y) {
            //     if matrix[y][x] == 0 { result_matrix[[y, x]] = 10 } else {
            //         result_matrix[[y, x]] = matrix[y][x];
            //     }
            // }
            // if visible_to_top(&matrix, x, y) {
            //     if matrix[y][x] == 0 { result_matrix[[y, x]] = 10 } else {
            //         result_matrix[[y, x]] = matrix[y][x];
            //     }
            // }
            // if visible_to_bottom(&matrix, x, y) {
            //     if matrix[y][x] == 0 { result_matrix[[y, x]] = 10 } else {
            //         result_matrix[[y, x]] = matrix[y][x];
            //     }
            // }
        }
    }
    println!("{:?} - max", max);

    // number of non-none elements
    // let mut count = 0;
    // for row in result_matrix.genrows() {
    //     for elem in row.iter() {
    //         if *elem != 0 {
    //             count += 1;
    //         }
    //     }
    // }

    // println!("{:?}", count);
    Ok(())
}

fn visible_to_left(matrix: &Vec<Vec<u32>>, x: usize, y: usize) -> i32 {
    let mut count = 0;
    for i in (0..x).rev() {
        if matrix[y][i] >= matrix[y][x] {
            return count+1;
        } else {
            count += 1;
        }
    }
    return count;
}

fn visible_to_right(matrix: &Vec<Vec<u32>>, x: usize, y: usize) -> i32 {
    let mut count = 0;
    for i in x + 1..matrix[0].len() {
        if matrix[y][i] >= matrix[y][x] {
            return count+1;
        } else {
            count += 1;
        }
    }
    return count;
}

fn visible_to_top(matrix: &Vec<Vec<u32>>, x: usize, y: usize) -> i32 {
    let mut count = 0;
    for i in (0..y).rev() {
        if matrix[i][x] >= matrix[y][x] {
            return count+1;
        } else {
            count += 1;
        }
    }
    return count;
}

fn visible_to_bottom(matrix: &Vec<Vec<u32>>, x: usize, y: usize) -> i32 {
    let mut count = 0;
    for i in y + 1..matrix.len() {
        if matrix[i][x] >= matrix[y][x] {
            return count+1;
        } else {
            println!("plus 1");
            count += 1;
        }
    }
    println!("end");
    return count;
}
