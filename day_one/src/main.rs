use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn load_from_file(file_path: &str) -> Vec<u32> {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    let numbers: Vec<u32> = reader
        .lines()
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .collect();

    numbers
}
fn main() {
    let filename: &str = "/Users/harryrose/Projects/advent_of_code_2021/day_one/data/input.txt";
    let nums: Vec<u32> = load_from_file(filename);

    let mut num_increases: u32 = 0;
    let mut prev: u32 = u32::MAX;
    for v in nums {
        if v > prev {
            num_increases += 1;
        }
        prev = v;
    }
    println!("{}", num_increases);
}