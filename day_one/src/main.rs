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

fn problem_one(nums: &[u32]) -> u32 {
    let mut num_increases: u32 = 0;
    let mut prev: &u32 = &u32::MAX;
    for v in nums {
        if v > prev {
            num_increases += 1;
        }
        prev = v;
    }
    num_increases
}

fn problem_two(nums: &[u32]) -> u32 {
    if nums.len() <= 3 {
        return 0;
    }

    let mut prev_sum = nums[0] + nums[1] + nums[2];
    let mut num_increases: u32 = 0;
    let mut curr_sum: u32;

    for i in 3..nums.len() {
        curr_sum = nums[i] + nums[i - 1] + nums[i - 2];
        if curr_sum > prev_sum {
            num_increases += 1;
        }
        prev_sum = curr_sum;
    }
    num_increases
}

fn main() {
    let filename: &str = "/Users/harryrose/Projects/advent_of_code_2021/day_one/data/input.txt";
    let nums: Vec<u32> = load_from_file(filename);

    println!("{}", problem_one(&nums));
    println!("{}", problem_two(&nums));
}
