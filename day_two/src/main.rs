use std::fs;

fn problem_one(horizontal_pos_one: &mut u32, depth_one: &mut u32, dir: &str, x: u32) {
    if dir == "forward" {
        *horizontal_pos_one += x;
    } else {
        if dir == "down" {
            *depth_one += x;
        } else {
            *depth_one -= x;
        }
    }
}

fn problem_two(
    horizontal_pos_two: &mut u32,
    depth_two: &mut u32,
    aim: &mut u32,
    dir: &str,
    x: u32,
) {
    if dir == "down" {
        *aim += x;
    } else if dir == "up" {
        *aim -= x;
    } else {
        *horizontal_pos_two += x;
        *depth_two += *aim * x;
    }
}

fn main() {
    // --snip--
    let filename: &str = "/Users/harryrose/Projects/advent_of_code_2021/data/day_two.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut horizontal_pos_one: u32 = 0;
    let mut depth_one: u32 = 0;

    let mut horizontal_pos_two: u32 = 0;
    let mut depth_two: u32 = 0;
    let mut aim: u32 = 0;

    for line in contents.lines() {
        let linevec: Vec<&str> = line.split_whitespace().collect();
        let dir = linevec[0];
        let x: u32 = linevec[1].parse().expect("Not a num");

        problem_one(&mut horizontal_pos_one, &mut depth_one, dir, x);
        problem_two(&mut horizontal_pos_two, &mut depth_two, &mut aim, dir, x);
    }
    let res_one = horizontal_pos_one * depth_one;
    println!("{}", res_one);

    let res_two = horizontal_pos_two * depth_two;
    println!("{}", res_two);
}
