const RADIX: u32 = 10;

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    part_one(buffer.clone());
    part_two(buffer.clone());
}

fn part_one(buffer: String) {
    let mut sum: u32 = 0;
    let mut char_iter = buffer.chars().peekable();
    while let Some(c) = char_iter.next() {
        match c.to_digit(RADIX) {
            Some(digit) => {
                match char_iter.peek() {
                    Some(next_char) => {
                        match next_char.to_digit(RADIX) {
                            Some(next_digit) => {
                                if next_digit == digit {
                                    sum = sum + digit;
                                }
                            },
                            None => {}
                        }
                    },
                    None => {}
                }
            },
            None => {}
        }
    }
    println!("{:?}", sum);
}

fn part_two(buffer: String) {
    let mut sum: u32 = 0;
    let digits: Vec<u32> = buffer
        .chars()
        .map(|c| match c.to_digit(RADIX) {
            Some(digit) => digit,
            None => 0
        })
        .collect();
    let length = digits.len() as isize;
    let steps: isize = length / 2;
    for i in 0..length {
        let mut new_idx = steps + i;
        if new_idx > length - 1 {
            new_idx = new_idx - length;
        }
        if digits[i as usize] == digits[new_idx as usize] {
            sum = sum + digits[i as usize];
        }
    }
    println!("{:?}", sum);
}
