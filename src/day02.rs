use std::fs;

struct Coords {
    horizontal: u32,
    depth: u32,
    aim: u32,
}

fn problem_one(coords: &mut Coords, line: &str) {
    let line_vec: Vec<&str> = line.split(" ").collect();

    let dir: &str = line_vec[0];
    let x: u32 = line_vec[1].parse::<u32>().expect("error");

    match &dir[..] {
        "forward" => coords.horizontal += x,
        "up" => coords.depth -= x,
        "down" => coords.depth += x,
        _ => println!("error"),
    }
}

fn problem_two(coords: &mut Coords, line: &str) {
    let line_vec: Vec<&str> = line.split(" ").collect();

    let dir: &str = line_vec[0];
    let x: u32 = line_vec[1].parse::<u32>().expect("error");

    match &dir[..] {
        "forward" => {
            coords.horizontal += x;
            coords.depth += coords.aim * x;
        }
        "up" => coords.aim -= x,
        "down" => coords.aim += x,
        _ => println!("error"),
    }
}

fn main() {
    let filename: &str = "/Users/harryrose/Projects/advent_of_code_2021/data/day_two.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut coords_one = Coords {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };

    let mut coords_two = Coords {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };

    for line in contents.lines() {
        problem_one(&mut coords_one, &line);
        problem_two(&mut coords_two, &line);
    }

    let res_one = coords_one.horizontal * coords_one.depth;
    println!("{}", res_one);

    let res_two = coords_two.horizontal * coords_two.depth;
    println!("{}", res_two);
}
