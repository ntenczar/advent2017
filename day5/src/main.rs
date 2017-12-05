use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Failed");
    let split_by_line: Vec<&str> = buffer.split("\n").collect();
    let digits_vec: Vec<i32> = split_by_line.iter().filter_map(|num_str| {
        return num_str.parse::<i32>().ok();
    }).collect();
    part_one(&mut digits_vec.clone());
    part_two(&mut digits_vec.clone());
}

fn part_one(digits_vec: &mut Vec<i32>) {
    let mut pos: i32 = 0;
    let mut total_steps = 0;
    while pos >= 0 && pos < digits_vec.len() as i32 {
        let jump = digits_vec[pos as usize];
        digits_vec[pos as usize] += 1;
        pos = pos + jump;
        total_steps += 1;
    }
    println!("{:?}", total_steps);
}

fn part_two(digits_vec: &mut Vec<i32>) {
    let mut pos: i32 = 0;
    let mut total_steps = 0;
    while pos >= 0 && pos < digits_vec.len() as i32 {
        let jump = digits_vec[pos as usize];
        if jump >= 3 {
            digits_vec[pos as usize] -= 1;
        } else {
            digits_vec[pos as usize] += 1;
        }
        pos = pos + jump;
        total_steps += 1;
    }
    println!("{:?}", total_steps);
}
