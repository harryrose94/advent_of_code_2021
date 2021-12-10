use std::collections::{HashMap, HashSet};
use std::fs;

#[macro_use]
extern crate lazy_static;

// Constants for problem
lazy_static! {
    static ref OPEN_CHARS: HashSet<char> = HashSet::from(['(', '{', '[', '<']);
    static ref CLOSE_CHARS: HashSet<char> = HashSet::from([')', '}', ']', '>']);
    static ref CHAR_PAIRS: HashMap<char, char> =
        HashMap::from([('(', ')'), ('{', '}'), ('[', ']'), ('<', '>')]);
    static ref ERROR_COSTS: HashMap<char, i64> =
        HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    static ref COMPLETION_COSTS: HashMap<char, i64> =
        HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);
}

fn compute_cost(s: &str) -> (i64, i64) {
    let s: Vec<char> = s.chars().collect();

    let mut open_stack: Vec<char> = Vec::new();
    for curr_char in s {
        // if an opening char, add to top of open-stack
        if OPEN_CHARS.contains(&curr_char) {
            open_stack.push(curr_char);
        } else if CLOSE_CHARS.contains(&curr_char) {
            // if a closing char, compare against most recent open
            if let Some(last_open) = open_stack.pop() {
                // fetch expected closing parenthesis
                match CHAR_PAIRS.get(&last_open) {
                    // if valid
                    Some(close_char) => {
                        if curr_char != *close_char {
                            match ERROR_COSTS.get(&curr_char) {
                                Some(cost) => return (*cost, 0),
                                _ => panic!("ERROR"),
                            }
                        }
                    }
                    // if not, exit
                    None => panic!("ERROR"),
                }
            }
        } else {
            panic!("ERROR");
        }
    }

    let mut completion_cost = 0;
    while let Some(open_char) = open_stack.pop() {
        completion_cost *= 5;
        match COMPLETION_COSTS.get(&open_char) {
            Some(cost) => completion_cost += cost,
            _ => panic!("ERROR"),
        }
    }
    (0, completion_cost)
}

fn problem_one_and_two(data: String) -> (i64, i64) {
    let mut error_cost = 0;
    let mut completion_costs: Vec<i64> = Vec::new();
    for line in data.lines() {
        let (e, c) = compute_cost(line);
        error_cost += e;
        if c > 0 {
            completion_costs.push(c);
        }
    }
    completion_costs.sort();
    let num_comp_costs = completion_costs.len();
    (error_cost, completion_costs[num_comp_costs / 2])
}
fn main() {
    let filename: &str = "/Users/harryrose/Projects/advent_of_code_2021/data/day10.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let (e, c) = problem_one_and_two(contents);
    println!("total costs {} {}", e, c);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_p1() {
        let filename: &str = "/Users/harryrose/Projects/advent_of_code_2021/data/day10.test.txt";
        let contents =
            super::fs::read_to_string(filename).expect("Something went wrong reading the file");
        let (ans1, ans2) = super::problem_one_and_two(contents);
        assert_eq!(ans1, 26397);
        assert_eq!(ans2, 288957);
    }
}
