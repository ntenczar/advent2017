extern crate regex;

use std::io::{self, BufRead};
use regex::Regex;
use std::collections::HashMap;

static INSTR_RE: &str = r"(\w+) (\w+) (-?\d+) if (\w+) (.+) (-?\d+)";

#[derive(Debug)]
struct CPU {
    registers: HashMap<String, isize>,
    largest: isize
}

impl CPU {
    fn new() -> CPU {
        return CPU {
            registers: HashMap::new(),
            largest: 0
        }
    }

    fn read_instruction(&mut self, instruction: &str) {
        let re: Regex = Regex::new(INSTR_RE).unwrap();
        let captures = re.captures(instruction).unwrap();
        let subj_reg = captures.get(1).unwrap().as_str();
        let inc_or_dec = captures.get(2).unwrap().as_str();
        let amount = captures.get(3).unwrap().as_str().parse::<isize>().unwrap();
        let check_reg = captures.get(4).unwrap().as_str();
        let operator = captures.get(5).unwrap().as_str();
        let check_amount = captures.get(6).unwrap().as_str().parse::<isize>().unwrap();

        if self.check(check_reg, operator, check_amount) {
            self.set_val(subj_reg, inc_or_dec, amount);
        }
    }

    fn get_val(&self, reg: &str) -> isize {
        match self.registers.get(reg) {
            Some(v) => *v,
            None => 0
        }
    }

    fn set_val(&mut self, subj_reg: &str, inc_or_dec: &str, amount: isize) {
        let existing_val = self.get_val(subj_reg);
        let reg = String::from(subj_reg);
        if inc_or_dec == "inc" {
            let set_val = existing_val + amount;
            self.registers.insert(reg, set_val);
            if set_val > self.largest {
                self.largest = set_val;
            }
        } else {
            let set_val = existing_val - amount;
            self.registers.insert(reg, set_val);
            if set_val > self.largest {
                self.largest = set_val;
            }
        }
    }

    fn check(&self, check_reg: &str, operator: &str, check_amount: isize) -> bool {
        let check_val = self.get_val(check_reg);
        match operator {
            "<" => check_val < check_amount,
            ">" => check_val > check_amount,
            "==" => check_val == check_amount,
            ">=" => check_val >= check_amount,
            "<=" => check_val <= check_amount,
            "!=" => check_val != check_amount,
            _ => panic!("Invalid operator!")
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut cpu = CPU::new();
    for res in stdin.lock().lines() {
        match res {
            Ok(line) => {
                cpu.read_instruction(&line);
            },
            Err(_) => {}
        }
    }
    println!("{:?}", cpu);
}
