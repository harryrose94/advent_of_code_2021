use std::collections::HashMap;
use std::fs;

pub fn main() {
    let filename: &str = "/Users/harryrose/Projects/advent_of_code_2021/data/day06.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let stack: Vec<u64> = contents
        .split(",")
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let mut counts: HashMap<u64, u64> = HashMap::new();

    // initialise all vals with 0
    for i in 0..9 {
        counts.insert(i, 0);
    }
    // count frequencies
    for x in stack {
        *counts.entry(x).or_insert(1) += 1;
    }

    for _ in 0..256 {
        let mut tmp_counts: HashMap<u64, u64> = HashMap::new();
        for i in 0..9 {
            tmp_counts.insert(i, 0);
        }

        let num_children = *counts.entry(0).or_insert(0);
        tmp_counts.insert(6, num_children);
        tmp_counts.insert(8, num_children);

        for (key, val) in &counts {
            if *key == { 0 } {
                continue;
            }
            *tmp_counts.entry(key - 1).or_insert(0) += val;
        }
        counts = tmp_counts;
    }
    let mut res = 0;
    for val in counts.values() {
        res += *val;
    }
    println!("{}", res);
}
