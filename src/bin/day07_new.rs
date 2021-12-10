use std::fs;

fn cost_p2(x: i64, i: i64) -> i64 {
    let y = (x - i).abs();
    y * (y + 1) / 2
}

fn median(mut arr: Vec<i64>) -> f64 {
    arr.sort();
    let len = arr.len();
    let mid = len / 2;
    if len % 2 == 0 {
        // even
        let s = arr[mid - 1] + arr[mid];
        s as f64 / 2.0
    } else {
        // odd
        arr[mid] as f64
    }
}

fn problem_one(stack: &Vec<i64>) -> i64 {
    let med = median(stack.clone());
    let med = med.round() as i64;
    let fuel = stack.iter().map(|&x| (x - med).abs()).sum::<i64>();
    fuel
}

// fn problem_two(stack: &Vec<i64>) -> f64 {
//     let applied_stack: Vec<i64> = stack.iter().map(|&x| x * (x + 1) / 2).collect();
//     let med = median(applied_stack.clone());
//     let v: f64 = applied_stack.iter().map(|&x| cost_p1(x as f64, med)).sum();
//     v
// }

fn read_data(filename: &str) -> Vec<i64> {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let stack: Vec<i64> = contents
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    stack
}
fn main() {
    let filename: &str = "/Users/harryrose/Projects/advent_of_code_2021/data/day07.test.txt";
    let stack = read_data(filename);
    let p1 = problem_one(&stack);
    println!("{}", p1);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_median() {
        let v1 = vec![1, 2, 3, 4, 5];
        assert_eq!(true, (super::median(v1) - 3.0).abs() < f64::EPSILON);

        let v2 = vec![1, 2, 4, 5];
        assert_eq!(true, (super::median(v2) - 3.0).abs() < f64::EPSILON);

        let v3 = vec![1, 2, 3];
        assert_eq!(true, (super::median(v3) - 2.0).abs() < f64::EPSILON);

        let v4 = vec![1, 10];
        assert_eq!(true, (super::median(v4) - 5.5).abs() < f64::EPSILON);

        let v5 = vec![1];
        assert_eq!(true, (super::median(v5) - 1.).abs() < f64::EPSILON);
    }
    #[test]
    fn test_problem_one() {
        let filename: &str = "/Users/harryrose/Projects/advent_of_code_2021/data/day07.test.txt";
        let stack = super::read_data(filename);
        let p1 = super::problem_one(&stack);
        assert_eq!(p1 as i64, 37);
    }
}
