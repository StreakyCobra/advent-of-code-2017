use std::fs::File;
use std::io::Read;
use std::process;
use std::collections::HashMap;
use std::cmp::max;

#[derive(Debug)]
enum Operation {
    Inc,
    Dec,
}

#[derive(Debug)]
enum Comparison {
    Lt,
    Lte,
    Equ,
    Nequ,
    Gte,
    Gt,
}

#[derive(Debug)]
struct Instruction<'a> {
    register: &'a str,
    operation: Operation,
    value: i32,
    cond_register: &'a str,
    cond_comparison: Comparison,
    cond_value: i32
}

/// Solve the eighth problem.
pub fn solve() {
    // Open the input file of the problem
    let mut file = match File::open("input/08.txt") {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Error: Input file for the problem 08 not found");
            process::exit(1)
        }
    };
    // Read the file in a String variable
    let mut content: String = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => (),
        Err(_) => {
            eprintln!("Error: Input file for the problem 08 can't be read");
            process::exit(1)
        }
    };
    let instructions = parse(&content);
    // Compute and print the solutions of the two parts
    println!("8. Solutions to the eighth problem:");
    println!("\tFirst part: {}", solve_first_part(&instructions));
    println!("\tSecond part: {}", solve_second_part(&instructions));
}

fn parse(content: &str) -> Vec<Instruction> {
    content.lines()
           .map(|line| {
               let mut iter = line.trim().split_whitespace();
               let register = iter.next().unwrap();
               let operation = match iter.next().unwrap().as_ref() {
                   "inc" => Operation::Inc,
                   "dec" => Operation::Dec,
                   _ => panic!("Wrong operation"),
               };
               let value = iter.next().unwrap().parse::<i32>().unwrap();
               iter.next();
               let cond_register = iter.next().unwrap();
               let cond_comparison = match iter.next().unwrap().as_ref() {
                   "<" => Comparison::Lt,
                   "<=" => Comparison::Lte,
                   "==" => Comparison::Equ,
                   "!=" => Comparison::Nequ,
                   ">=" => Comparison::Gte,
                   ">" => Comparison::Gt,
                   _ => panic!("Wrong comparison"),
               };
               let cond_value = iter.next().unwrap().parse::<i32>().unwrap();
               Instruction {
                   register,
                   operation,
                   value,
                   cond_register,
                   cond_comparison,
                   cond_value
               }
           }).collect()
}

fn solve_generic(instructions: &Vec<Instruction>) -> (i32, i32) {
    let mut registers: HashMap<String, i32> = HashMap::new();
    let mut all_max: i32 = 0;
    for instruction in instructions {
        let cond_reg_value = match registers.get(instruction.cond_register) {
            Some(val) => *val,
            None => 0,
        };
        let valid = match instruction.cond_comparison {
            Comparison::Lt => cond_reg_value < instruction.cond_value,
            Comparison::Lte => cond_reg_value <= instruction.cond_value,
            Comparison::Equ => cond_reg_value == instruction.cond_value,
            Comparison::Nequ => cond_reg_value != instruction.cond_value,
            Comparison::Gte => cond_reg_value >= instruction.cond_value,
            Comparison::Gt => cond_reg_value > instruction.cond_value,
        };
        if valid {
            let reg_value = match registers.get(instruction.register) {
                Some(val) => *val,
                None => 0,
            };
            let regname: String = String::from(instruction.register);
            match instruction.operation {
                Operation::Inc => registers.insert(regname, reg_value + instruction.value),
                Operation::Dec => registers.insert(regname, reg_value - instruction.value),
            };
        };
        all_max = max(all_max, *registers.values().max().unwrap());
    }
    (*registers.values().max().unwrap(), all_max)
}

fn solve_first_part(instructions: &Vec<Instruction>) -> i32 {
    solve_generic(instructions).0
}

fn solve_second_part(instructions: &Vec<Instruction>) -> i32 {
    solve_generic(instructions).1
}

#[cfg(test)]
mod tests {

    use super::{parse, solve_first_part, solve_second_part};

    const INPUT: &str = "b inc 5 if a > 1
                         a inc 1 if b < 5
                         c dec -10 if a >= 1
                         c inc -20 if c == 10";

    #[test]
    fn eighth_problem_first_part() {
        assert_eq!(solve_first_part(&parse(INPUT)), 1);
    }

    #[test]
    fn eighth_problem_second_part() {
        assert_eq!(solve_second_part(&parse(INPUT)), 10);
    }

}
