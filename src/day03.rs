use std::fs;
static NUM_BITS: usize = 12;

fn problem_one(lines: &[&str]) -> usize {
    let num_lines = lines.len();
    let mut res = vec![0; NUM_BITS];
    for line in lines {
        for (i, c) in line.chars().enumerate() {
            res[i] += c.to_digit(10).unwrap();
        }
    }

    let mut gamma: usize = 0;
    let mut epsilon: usize = 0;
    for (i, x) in res.iter().rev().enumerate() {
        if *x > (num_lines / 2).try_into().unwrap() {
            gamma += usize::pow(2, i.try_into().unwrap());
        } else {
            epsilon += usize::pow(2, i.try_into().unwrap());
        }
    }
    gamma * epsilon
}

fn problem_two_helper(lines: &[&str], msb: bool) -> String {
    let mut stack: Vec<&str> = lines.to_vec().clone();
    let mut idx: usize = 0;

    while stack.len() > 1 {
        let mut one_stack: Vec<&str> = Vec::new();
        let mut zero_stack: Vec<&str> = Vec::new();

        let mut num_ones = 0;
        let mut num_zeros = 0;

        while let Some(top) = stack.pop() {
            let digit = top.chars().nth(idx).unwrap().to_digit(10).unwrap();

            if digit == 1 {
                one_stack.push(&top);
                num_ones += 1;
            } else {
                zero_stack.push(&top);
                num_zeros += 1;
            }
        }

        if msb {
            // if 1 is the most significant digit
            if num_ones >= num_zeros {
                stack = one_stack.clone();
            } else {
                stack = zero_stack.clone();
            }
        } else {
            // if 1 is the least significant digit
            if num_ones < num_zeros {
                stack = one_stack.clone();
            } else {
                stack = zero_stack.clone();
            }
        }
        idx += 1;
    }

    let mut res = String::new();
    if let Some(res_chars) = stack.pop() {
        res.push_str(res_chars);
    }
    res
}

fn problem_two(lines: &[&str]) -> usize {
    let oxy = problem_two_helper(&lines, true);
    let carbon = problem_two_helper(&lines, false);

    let mut oxy_int: usize = 0;
    for (i, x) in oxy.chars().rev().enumerate() {
        let digit = x.to_digit(10).unwrap();
        if digit == 1 {
            oxy_int += usize::pow(2, i.try_into().unwrap());
        }
    }
    let mut carbon_int: usize = 0;
    for (i, x) in carbon.chars().rev().enumerate() {
        let digit = x.to_digit(10).unwrap();
        if digit == 1 {
            carbon_int += usize::pow(2, i.try_into().unwrap());
        }
    }
    oxy_int * carbon_int
}

fn main() {
    let filename: &str = "/Users/harryrose/Projects/advent_of_code_2021/data/day_three.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();
    let p1 = problem_one(&lines);
    println!("{}", p1);

    let p2 = problem_two(&lines);
    println!("{}", p2);
}

#[cfg(test)]
mod tests {
    #[test]
    fn d04_first() {
        let input =
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";
        let lines: Vec<&str> = input.split("\n").collect();
        let res = super::problem_two(&lines);
        assert_eq!(res, 230);
    }
}
