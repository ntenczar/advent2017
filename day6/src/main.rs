use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Failed");
    let mut input: Vec<usize> = buffer.split_whitespace().filter_map(|num_str| {
        return num_str.parse::<usize>().ok();
    }).collect();
    let mut seen: Vec<Vec<usize>> = vec![];
    while !seen.contains(&input) {
        seen.push(input.clone());
        redistribute(&mut input);
    }
    let mut seen_idx = 0;
    for (i, s) in seen.iter().enumerate() {
        if *s == input {
            seen_idx = i;
        }
    }
    println!("{:?}", seen.len());
    println!("{:?}", seen.len() - seen_idx);
}

fn redistribute(input: &mut Vec<usize>) {
    let max_idx: usize = input.iter().enumerate().fold(0 as usize, |acc, (idx, compare)| {
        if *compare > input[acc] {
            return idx;
        } else {
            return acc;
        }
    });
    let num = input[max_idx];
    input[max_idx] = 0;
    for i in 0..num {
        let idx = (max_idx + i + 1) % input.len();
        input[idx] += 1;
    }
}
