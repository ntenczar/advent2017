use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Failed");
    let mut split_by_line: Vec<&str> = buffer.split("\n").collect();
    split_by_line.pop(); // drop newline
    let passphrases: Vec<Vec<&str>> = split_by_line.iter().map(|l| {
        return l.split(" ").collect();
    }).collect();
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
