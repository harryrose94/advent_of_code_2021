use std::collections::HashSet;
use std::fs;

fn parse_grid(raw_data: &str) -> Vec<Vec<u32>> {
    let mut grid: Vec<Vec<u32>> = Vec::new();
    for line in raw_data.lines() {
        let mut row: Vec<u32> = Vec::new();
        for x in line.chars() {
            if let Some(digit) = x.to_digit(10) {
                row.push(digit);
            }
        }
        grid.push(row);
    }
    grid
}

fn is_min(x: u32, v: Vec<u32>) -> bool {
    let min_nb = v.iter().min().unwrap();
    x < *min_nb
}

fn problem_one(grid: &Vec<Vec<u32>>) -> u32 {
    let mut total_height = 0;
    for (r_idx, row) in grid.iter().enumerate() {
        for (c_idx, x) in row.iter().enumerate() {
            let nbs = get_nbs(grid, r_idx, c_idx);
            let mut nbs_to_eval: Vec<u32> = Vec::new();
            for (nb_x, nb_y) in nbs.iter() {
                nbs_to_eval.push(grid[*nb_x][*nb_y]);
            }
            if is_min(*x, nbs_to_eval) {
                total_height += 1 + x;
            };
        }
    }
    total_height
}

fn get_nbs(grid: &Vec<Vec<u32>>, r_idx: usize, c_idx: usize) -> Vec<(usize, usize)> {
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    let first_row = r_idx == 0;
    let last_row = r_idx == (num_rows - 1);

    let first_col = c_idx == 0;
    let last_col = c_idx == (num_cols - 1);

    let nbs: Vec<(usize, usize)>;

    if first_row & first_col {
        nbs = vec![(r_idx + 1, c_idx), (r_idx, c_idx + 1)];
    } else if first_row & last_col {
        nbs = vec![(r_idx + 1, c_idx), (r_idx, c_idx - 1)];
    } else if last_row & first_col {
        nbs = vec![(r_idx - 1, c_idx), (r_idx, c_idx + 1)];
    } else if last_row & last_col {
        nbs = vec![(r_idx - 1, c_idx), (r_idx, c_idx - 1)];
    } else if first_row {
        nbs = vec![(r_idx + 1, c_idx), (r_idx, c_idx - 1), (r_idx, c_idx + 1)];
    } else if last_row {
        nbs = vec![(r_idx - 1, c_idx), (r_idx, c_idx - 1), (r_idx, c_idx + 1)];
    } else if first_col {
        nbs = vec![(r_idx - 1, c_idx), (r_idx + 1, c_idx), (r_idx, c_idx + 1)];
    } else if last_col {
        nbs = vec![(r_idx - 1, c_idx), (r_idx + 1, c_idx), (r_idx, c_idx - 1)];
    } else {
        nbs = vec![
            (r_idx - 1, c_idx),
            (r_idx + 1, c_idx),
            (r_idx, c_idx - 1),
            (r_idx, c_idx + 1),
        ];
    }
    nbs
}

fn problem_two(grid: &Vec<Vec<u32>>) -> u32 {
    let num_rows = grid.len();
    let num_cols = grid[0].len();
    let mut island_grid: Vec<Vec<u32>> = vec![vec![0; num_cols]; num_rows];
    let mut basin_sizes: Vec<usize> = Vec::new();

    for (r_idx, row) in grid.iter().enumerate() {
        for (c_idx, x) in row.iter().enumerate() {
            let nbs = get_nbs(grid, r_idx, c_idx);
            let mut nbs_to_eval: Vec<u32> = Vec::new();
            for (nb_x, nb_y) in nbs.iter() {
                nbs_to_eval.push(grid[*nb_x][*nb_y]);
            }
            if is_min(*x, nbs_to_eval) {
                let mut visited: HashSet<(usize, usize)> = HashSet::new();
                let mut stack: Vec<(usize, usize)> = Vec::new();
                stack.push((r_idx, c_idx));

                while let Some((top_x, top_y)) = stack.pop() {
                    visited.insert((top_x, top_y));
                    let curr_val = grid[top_x][top_y];
                    island_grid[top_x][top_y] = 1;

                    let next_nbs = get_nbs(grid, top_x, top_y);
                    for (nb_x, nb_y) in next_nbs {
                        let nb_val = grid[nb_x][nb_y];
                        if !visited.contains(&(nb_x, nb_y)) && (nb_val > curr_val) && (nb_val != 9)
                        {
                            stack.push((nb_x, nb_y));
                        }
                    }
                }
                basin_sizes.push(visited.len());
            }
        }
    }
    basin_sizes.sort();
    basin_sizes
        .iter()
        .rev()
        .take(3)
        .map(|x| *x as u32)
        .product()
}

fn main() {
    let filename: &str = "/Users/harryrose/Projects/advent_of_code_2021/data/day09.txt";
    let raw_data = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let grid = parse_grid(&raw_data);
    println!("total height {}", problem_one(&grid));
    println!("total size {}", problem_two(&grid));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_p1() {
        let filename: &str = "/Users/harryrose/Projects/advent_of_code_2021/data/day09.test.txt";
        let raw_data =
            super::fs::read_to_string(filename).expect("Something went wrong reading the file");
        let grid = super::parse_grid(&raw_data);
        let p1 = super::problem_one(&grid);
        assert_eq!(p1, 15);
    }
    #[test]
    fn test_p2() {
        let filename: &str = "/Users/harryrose/Projects/advent_of_code_2021/data/day09.test.txt";
        let raw_data =
            super::fs::read_to_string(filename).expect("Something went wrong reading the file");
        let grid = super::parse_grid(&raw_data);
        let p2 = super::problem_two(&grid);
        assert_eq!(p2, 1134);
    }
}
