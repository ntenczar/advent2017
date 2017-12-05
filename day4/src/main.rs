use std::io::{self, Read};

trait FuzzyCompare {
    fn fuzzy_equal(&self, &str) -> bool;
}

impl FuzzyCompare for str {
    fn fuzzy_equal(&self, other: &str) -> bool {
        if self.len() != other.len() {
            return false;
        }
        let mut other = String::from(other);
        for c in self.chars() {
            let mut idx_to_remove: Option<usize> = None;
            for (idx, compare_char) in other.chars().enumerate() {
                if compare_char == c {
                    idx_to_remove = Some(idx);
                }
            }
            // surely there is a more idiomatic way to do this
            match idx_to_remove {
                Some(i) => other.remove(i),
                None => 'a'
            };
        }
        if other.len() == 0 {
            return true;
        }
        return false;
    }
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Failed");
    let mut split_by_line: Vec<&str> = buffer.split("\n").collect();
    split_by_line.pop(); // drop newline
    let passphrases: Vec<Vec<&str>> = split_by_line.iter().map(|l| {
        return l.split(" ").collect();
    }).collect();
    part_one(passphrases.clone());
    part_two(passphrases.clone());
}

fn part_one(passphrases: Vec<Vec<&str>>) {
    let mut num_valid = 0;
    for passphrase in passphrases {
        let mut row_valid = true;
        for i in 0..passphrase.len() {
            let partial = passphrase[i];

            for j in 0..passphrase.len() {
                let compare = passphrase[j];

                if i != j && compare == partial {
                    row_valid = false;
                }
            }
        }
        if row_valid {
            num_valid += 1;
        }
    }
    println!("{:?}", num_valid);
}

fn part_two(passphrases: Vec<Vec<&str>>) {
    let mut num_valid = 0;
    for passphrase in passphrases {
        let mut row_valid = true;
        for i in 0..passphrase.len() {
            let partial = passphrase[i];

            for j in 0..passphrase.len() {
                let compare = passphrase[j];

                if i != j && compare.fuzzy_equal(partial) {
                    row_valid = false;
                }
            }
        }
        if row_valid {
            num_valid += 1;
        }
    }
    println!("{:?}", num_valid);
}
