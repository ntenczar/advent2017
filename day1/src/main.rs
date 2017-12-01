fn main() {
    const RADIX: u32 = 10;
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
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
