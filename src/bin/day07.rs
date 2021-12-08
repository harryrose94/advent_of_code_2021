use std::fs;

fn cost_p1(x: i64, i: i64) -> i64 {
    let y = (x - i).abs();
    y
}

fn cost_p2(x: i64, i: i64) -> i64 {
    let y = (x - i).abs();
    y * (y + 1) / 2
}

pub fn main() {
    let filename: &str = "/Users/harryrose/Projects/advent_of_code_2021/data/day07.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let stack: Vec<i64> = contents
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let min_val: i64 = *stack.iter().min().unwrap_or(&0);
    let max_val: i64 = *stack.iter().max().unwrap_or(&0);

    let mut min_so_far = i64::MAX;
    for i in min_val..(max_val + 1) {
        let r: i64 = stack.iter().map(|x| cost_p2(*x, i)).sum();
        if r < min_so_far {
            min_so_far = r;
        }
    }

    println!("{}", min_so_far);
}
