use std::io::{self, BufRead};

/*
static REPLACE_EXCL_RE: &str = r"!.";
static REPLACE_GARBAGE_RE: &str = r#"<[,./<\w{}"\\' ]*>"#;
static FIND_GROUPS_RE: &str = r#"{((?:{+}*,?)*)}"#;
*/

fn main() {
    let stdin = io::stdin();
    for res in stdin.lock().lines() {
        match res {
            Ok(line) => parse_line(line),
            Err(_) => {}
        }
    }
}

fn parse_line(line: String) {
    let mut is_banged = false;
    let mut in_garbage = false;
    let mut num_garbage = 0;
    let mut depth = 0;
    let mut count = 0;
    for c in line.chars() {
        if is_banged {
            is_banged = false;
            continue;
        }
        if in_garbage && c != '>' && c != '!' {
            num_garbage += 1;
        }
        match c {
            '{' => {
                if !in_garbage {
                    depth += 1;
                }
            },
            '}' => {
                if !in_garbage {
                    count += depth;
                    depth -= 1;
                }
            },
            '!' => {
                if !is_banged {
                    is_banged = true;
                }
            },
            '<' => {
                in_garbage = true;
            },
            '>' => {
                in_garbage = false;
            },
            _ => {}
        }
    }
    println!("{:?}", count);
    println!("{:?}", num_garbage);
}
