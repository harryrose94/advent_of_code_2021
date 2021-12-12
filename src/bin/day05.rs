use std::collections::HashMap;
use std::fs;
fn main() {
    let filename: &str = "/Users/harryrose/Projects/advent_of_code_2021/data/day05.txt";
    let raw_data = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut hashmap: HashMap<(i32, i32), i32> = HashMap::new();

    for line in raw_data.lines() {
        let splits = line.split(" -> ");
        let mut coords: [Vec<i32>; 2] = [vec![], vec![]];
        for (i, split) in splits.enumerate() {
            let mut point: Vec<i32> = Vec::new();
            let point_str = split.split(',');
            for x in point_str {
                point.push(x.parse::<i32>().unwrap());
            }
            assert_eq!(point.len(), 2);
            coords[i] = point;
        }

        let x0 = coords[0][0];
        let y0 = coords[0][1];

        let x1 = coords[1][0];
        let y1 = coords[1][1];

        let xd: i32 = x1 - x0;
        let yd: i32 = y1 - y0;

        let diff: (i32, i32) = (xd, yd);

        match diff {
            (0, _) => {
                let it = if yd > 0 { 0..=yd } else { yd..=0 };
                for i in it {
                    *hashmap.entry((x0, y0 + i)).or_insert(0) += 1;
                }
            }
            (_, 0) => {
                let it = if xd > 0 { 0..=xd } else { xd..=0 };
                for i in it {
                    *hashmap.entry((x0 + i, y0)).or_insert(0) += 1;
                }
            }
            (_, _) => {
                let it_x = 0..=xd.abs();
                let it_y = 0..=yd.abs();

                let x_sign = if xd > 0 { 1 } else { -1 };
                let y_sign = if yd > 0 { 1 } else { -1 };

                for (i, j) in it_x.zip(it_y) {
                    *hashmap
                        .entry((x0 + i * x_sign, y0 + j * y_sign))
                        .or_insert(0) += 1;
                }
            }
        }
    }
    let mut ans: u32 = 0;
    for v in hashmap.values() {
        if v >= &2 {
            ans += 1;
        }
    }
    println!("{:?}", ans);
}
