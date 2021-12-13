use std::cmp::min;
use std::fs;

const FIRST_X: u32 = 655;
const FIRST_Y: u32 = 447;

fn display_output(output_vec: Vec<u32>, num_rows: u32, num_cols: u32) {
    for i in 0..num_rows {
        let start_idx = (i * num_cols) as usize;
        let end_idx = ((i + 1) * num_cols) as usize;
        let data_to_print = output_vec.get(start_idx..end_idx).unwrap();

        let mut formatted_vec: Vec<&str> = Vec::new();
        for c in data_to_print {
            match c {
                1 => formatted_vec.push("+"),
                0 => formatted_vec.push(" "),
                _ => panic!("ERROR"),
            }
        }
        let line_to_print = formatted_vec.join(" ");
        println!("{}", line_to_print);
    }
}

fn main() {
    let filename: &str = "/Users/harryrose/Projects/advent_of_code_2021/data/day13.txt";
    let raw_data = fs::read_to_string(filename).expect("Error reading the file");

    let mut folds: Vec<&str> = Vec::new();

    let mut xs: Vec<u32> = Vec::new();
    let mut ys: Vec<u32> = Vec::new();
    for line in raw_data.lines() {
        if line.starts_with("fold") {
            folds.push(line);
        } else if line.is_empty() {
            continue;
        } else {
            let coords: Vec<&str> = line.split(',').collect();
            assert_eq!(coords.len(), 2);
            let x: u32 = coords[0].parse().unwrap();
            let y: u32 = coords[1].parse().unwrap();
            xs.push(x);
            ys.push(y);
        }
    }

    let mut nx = (FIRST_X * 2) + 1;
    let mut ny = (FIRST_Y * 2) + 1;
    let capacity = nx * ny;
    let mut grid_hor = vec![0; capacity as usize];
    let mut grid_ver = vec![0; capacity as usize];

    for (x, y) in xs.iter().zip(ys.iter()) {
        let z_hor: u32 = (y * nx) + x;
        grid_hor[z_hor as usize] = 1;

        let z_ver: u32 = (x * ny) + y;
        grid_ver[z_ver as usize] = 1;
    }

    for fold in folds {
        let fold = fold.replace("fold along ", "");
        let inx: Vec<&str> = fold.split('=').collect();
        let state = inx[0];
        let pos: u32 = inx[1].parse().unwrap();

        match state {
            "x" => {
                let s = (pos * ny) as usize;
                let e = ((pos + 1) * ny) as usize;
                let start_vec = grid_ver.get(0..s).unwrap();
                let end_vec = grid_ver.get(e..).unwrap();
                assert_eq!(start_vec.len(), end_vec.len());

                let mut grid_hor_tmp = vec![0; end_vec.len()];
                let mut grid_ver_tmp = vec![0; end_vec.len()];

                nx /= 2;
                for i in 0..nx {
                    for j in 0..ny {
                        let p_ver: usize = ((i * ny) + j) as usize;
                        let q_ver: usize = (((nx - i - 1) * ny) + j) as usize;
                        grid_ver_tmp[p_ver] = min(start_vec[p_ver] + end_vec[q_ver], 1);

                        // update horizontal representation
                        let p_hor: usize = ((j * nx) + i) as usize;
                        grid_hor_tmp[p_hor] = grid_ver_tmp[p_ver];
                    }
                }
                grid_hor = grid_hor_tmp.clone();
                grid_ver = grid_ver_tmp.clone();
            }
            "y" => {
                let s = (pos * nx) as usize;
                let e = ((pos + 1) * nx) as usize;
                let start_vec = grid_hor.get(0..s).unwrap();
                let end_vec = grid_hor.get(e..).unwrap();
                assert_eq!(start_vec.len(), end_vec.len());

                let mut grid_hor_tmp = vec![0; end_vec.len()];
                let mut grid_ver_tmp = vec![0; end_vec.len()];

                ny /= 2;
                for i in 0..nx {
                    for j in 0..ny {
                        let p: usize = ((j * nx) + i) as usize;
                        let q: usize = (((ny - j - 1) * nx) + i) as usize;
                        grid_hor_tmp[p] = min(start_vec[p] + end_vec[q], 1);

                        // update vertical representation
                        let p_ver = ((i * ny) + j) as usize;
                        grid_ver_tmp[p_ver] = grid_hor_tmp[p];
                    }
                }
                grid_hor = grid_hor_tmp.clone();
                grid_ver = grid_ver_tmp.clone();
            }
            _ => panic!("ERROR"),
        }
    }
    let num_dots: u32 = grid_hor.iter().sum();
    println!("Num dots {}", num_dots);

    // Display result
    display_output(grid_hor, ny, nx);
}
