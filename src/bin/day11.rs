use ndarray::prelude::*;
use ndarray::Array;
use std::collections::HashSet;
use std::fs;

const NUM_STEPS: u32 = 500;

fn get_nbs(i: usize, j: usize, num_rows: usize, num_cols: usize) -> Vec<(usize, usize)> {
    let mut nbs: Vec<(usize, usize)> = Vec::new();
    if (i + 1 < num_rows) && (j < num_cols) {
        nbs.push((i + 1, j));
    }
    if (0 < i) && (j < num_cols) {
        nbs.push((i - 1, j));
    }

    if (i < num_rows) && (j + 1 < num_cols) {
        nbs.push((i, j + 1));
    }
    if (i < num_rows) && (0 < j) {
        nbs.push((i, j - 1));
    }

    if (0 < i) && (0 < j) {
        nbs.push((i - 1, j - 1));
    }
    if (i + 1 < num_rows) && (j + 1 < num_cols) {
        nbs.push((i + 1, j + 1));
    }

    if (0 < i) && (j + 1 < num_cols) {
        nbs.push((i - 1, j + 1));
    }
    if (i + 1 < num_rows) && (0 < j) {
        nbs.push((i + 1, j - 1));
    }
    nbs
}

fn main() {
    let filename: &str = "/Users/harryrose/Projects/advent_of_code_2021/data/day11.txt";
    let raw_data = fs::read_to_string(filename).expect("Error reading the file");
    let num_cols = raw_data.lines().nth(1).unwrap().len();
    let full_stream: String = raw_data.lines().collect();
    let num_rows = full_stream.len() / num_cols;
    println!("{:?} {:?}", num_rows, num_cols);

    let data: Vec<u32> = full_stream
        .chars()
        .map(|x| x.to_digit(10).expect("Should be a number"))
        .collect();

    let mut grid = Array::from(data)
        .into_shape((num_rows, num_cols))
        .expect("err");

    let mut total_flashes = 0;

    for step in 0..NUM_STEPS {
        let mut loop_ctr = 0;

        grid += 1;
        let mut flashes_to_process: Vec<(usize, usize)> = Vec::new();
        let mut flashes_processed: HashSet<(usize, usize)> = HashSet::new();

        while loop_ctr == 0 || !flashes_to_process.is_empty() {
            while let Some((fx, fy)) = flashes_to_process.pop() {
                grid.slice_mut(s![fx, fy]).fill(0);
                flashes_processed.insert((fx, fy));

                let nbs = get_nbs(fx, fy, num_rows, num_cols);
                for nb in nbs {
                    if !flashes_processed.contains(&nb) {
                        grid[[nb.0, nb.1]] += 1;
                    }
                }
            }

            for (idx, val) in grid.indexed_iter() {
                if val > &9 {
                    flashes_to_process.push(idx);
                }
            }
            loop_ctr += 1;
        }
        // part 2
        total_flashes += flashes_processed.len();
        if flashes_processed.len() == num_cols * num_rows {
            println!("all flashed {}", step);
            break;
        }
    }
    println!("num flashes {}", total_flashes);
}
