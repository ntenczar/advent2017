#![feature(plugin)]
#![plugin(quickcheck_macros)]

#[cfg(test)]
extern crate quickcheck;

use std::io::{self, BufRead};

const ROUNDS: usize = 64;

fn main() {
    let stdin = io::stdin();
    for res in stdin.lock().lines() {
        match res {
            Ok(line) => {
                // part_one(line.clone());
                part_two(line);
            },
            Err(_) => {}
        }
    }
}

fn part_one(line: String) {
    let input: Vec<usize> = line.split(",").map(|s| {
        return s.parse::<usize>().unwrap();
    }).collect();
    let hashed = hash::hash((0..256).collect(), input, &mut 0, &mut 0);
    println!("Multiplied: {:?}", hashed[0]*hashed[1]);
}

fn part_two(line: String) {
    let mut lengths: Vec<usize> = vec![];
    for byte in line.as_bytes() {
        lengths.push(*byte as usize);
    }
    // 17, 31, 73, 47, 23
    lengths.push(17);
    lengths.push(31);
    lengths.push(73);
    lengths.push(47);
    lengths.push(23);
    let pos: &mut usize = &mut 0;
    let skip: &mut usize = &mut 0;
    let seq: Vec<usize> = (0..256).collect();
    let mut sparse_hash = seq.clone();
    for _round in 0..ROUNDS {
        sparse_hash = hash::hash(sparse_hash, lengths.clone(), pos, skip);
    }
    let dense_hash: Vec<u8> = hash::sparse_to_dense(sparse_hash);
    println!("{:?}", dense_hash);
    let formatted = dense_hash.iter().fold(String::from(""), |acc, d| {
        let hex_str;
        if *d < 10 {
            hex_str = format!("0{:x}", d);
        } else {
            hex_str = format!("{:x}", d);
        }
        return acc + &hex_str;
    });
    println!("Formatted: {:?}", formatted);
}

mod hash {
    pub fn hash(list: Vec<usize>, lengths: Vec<usize>,
                pos: &mut usize, skip: &mut usize) -> Vec<usize> {
        let mut hashed_list = list.clone();
        for length in lengths.iter() {
            hashed_list = reverse_chunk(hashed_list.clone(), *pos, *length);
            *pos = *pos + length + *skip;
            *pos %= hashed_list.len();
            *skip += 1;
        }
        return hashed_list;
    }

    pub fn reverse_chunk(list: Vec<usize>, pos: usize, length: usize) -> Vec<usize>{
        let num_items = list.len();
        if num_items == 0 {
            return list;
        }
        let range = pos..(length + pos);
        let mut chunk: Vec<usize> = range.clone().map(|i| list[i % num_items]).collect();
        chunk.reverse();
        let mut new_list = list.clone();
        let mut chunk_idx = 0;
        for i in range {
            new_list[i % num_items] = chunk[chunk_idx];
            chunk_idx += 1;
        }
        return new_list;
    }

    pub fn sparse_to_dense(sparse: Vec<usize>) -> Vec<u8> {
        let mut dense_hash: Vec<u8> = vec![];
        for n in 0..16 {
            let start_idx = n*16;
            let end_idx = n*16 + 16;
            let val: u8 =
                (start_idx+1..end_idx).fold(sparse[start_idx] as u8, |acc, i| {
                    return acc as u8 ^ sparse[i] as u8;
                });
            dense_hash.push(val);
        }
        return dense_hash;
    }
}

#[cfg(test)]
mod tests {
    use hash;

    #[test]
    fn reverse_chunk_works() {
        let list = vec![0,1,2,3,4,5];
        assert_eq!(hash::reverse_chunk(list.clone(), 2, 4), vec![0,1,5,4,3,2]);
        assert_eq!(hash::reverse_chunk(list.clone(), 0, 3), vec![2,1,0,3,4,5]);
        assert_eq!(hash::reverse_chunk(list.clone(), 4, 3), vec![4,1,2,3,0,5]);
        assert_eq!(hash::reverse_chunk(list.clone(), 1, 6), vec![1,0,5,4,3,2]);
        assert_eq!(hash::reverse_chunk(list.clone(), 0, 6), vec![5,4,3,2,1,0]);
        assert_eq!(hash::reverse_chunk(list.clone(), 5, 2), vec![5,1,2,3,4,0]);
        assert_eq!(hash::reverse_chunk(list, 6, 2), vec![1,0,2,3,4,5]);
    }

    #[quickcheck]
    fn reverse_chunk_quickcheck(xs: Vec<usize>, pos: usize, len: usize) -> bool {
        xs == hash::reverse_chunk(
            hash::reverse_chunk(xs.clone(), pos, len), pos, len)
    }

    #[test]
    fn sparse_to_dense_works() {
        let sparse: Vec<usize> = vec![
            65,27,9,1,4,3,40,50,91,7,6,0,2,5,68,22,
            65,27,9,1,4,3,40,50,91,7,6,0,2,5,68,22,
            65,27,9,1,4,3,40,50,91,7,6,0,2,5,68,22,
            65,27,9,1,4,3,40,50,91,7,6,0,2,5,68,22,
            65,27,9,1,4,3,40,50,91,7,6,0,2,5,68,22,
            65,27,9,1,4,3,40,50,91,7,6,0,2,5,68,22,
            65,27,9,1,4,3,40,50,91,7,6,0,2,5,68,22,
            65,27,9,1,4,3,40,50,91,7,6,0,2,5,68,22,
            65,27,9,1,4,3,40,50,91,7,6,0,2,5,68,22,
            65,27,9,1,4,3,40,50,91,7,6,0,2,5,68,22,
            65,27,9,1,4,3,40,50,91,7,6,0,2,5,68,22,
            65,27,9,1,4,3,40,50,91,7,6,0,2,5,68,22,
            65,27,9,1,4,3,40,50,91,7,6,0,2,5,68,22,
            65,27,9,1,4,3,40,50,91,7,6,0,2,5,68,22,
            65,27,9,1,4,3,40,50,91,7,6,0,2,5,68,22,
            65,27,9,1,4,3,40,50,91,7,6,0,2,5,68,22,
         ];
        assert_eq!(hash::sparse_to_dense(sparse),
                   vec!(64,64,64,64,64,64,64,64,64,64,64,64,64,64,64,64));
    }
}
