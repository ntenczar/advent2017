use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Failed");
    let mut split_by_line: Vec<&str> = buffer.split("\n").collect();
    split_by_line.pop(); // drop newline
    let full_matrix: Vec<Vec<u32>> = split_by_line.iter().map(|l| {
        return l.split("\t").map(|num_str| {
            return num_str.parse::<u32>().unwrap();
        }).collect();
    }).collect();
    part_one(full_matrix.clone());
    part_two(full_matrix.clone());
}

fn part_one(matrix: Vec<Vec<u32>>) {
    let mut row_sums = 0;
    for row_i in 0..matrix.len() {
        let row: Vec<u32> = matrix[row_i].clone();
        let row_max = row.iter().fold(0, |acc, &num| {
            if acc >= num {
                return acc;
            }
            return num;
        });
        let row_min = row.iter().fold(row[0], |acc, &num| {
            if num <= acc {
                return num;
            }
            return acc;
        });
        let row_diff = row_max - row_min;
        row_sums += row_diff;
    }
    println!("{:?}", row_sums);
}

fn part_two(matrix: Vec<Vec<u32>>) {
    let mut row_sums = 0;
    for row_i in 0..matrix.len() {
        let row: Vec<u32> = matrix[row_i].clone();
        let mut row_val = 0;
        for i in 0..row.len() {
            let val = row[i];

            for j in 0..row.len() {
                let compare = row[j];

                if i == j {
                    continue;
                }

                if val % compare == 0 {
                    row_val = val / compare;
                }
            }
        }
        row_sums += row_val;
    }
    println!("{:?}", row_sums);
}
