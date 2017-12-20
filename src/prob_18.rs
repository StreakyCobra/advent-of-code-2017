use std::fs::File;
use std::io::Read;
use std::process;
use std::collections::HashMap;

type Program = Vec<Instruction>;

#[derive(Debug, Clone)]
enum Val {
    Int(isize),
    Reg(char),
}

impl Val {
    fn new(value: &str) -> Val {
        match value.chars().nth(0) {
            Some(v) if v.is_digit(10) || v == '-' => Val::Int(value.parse::<isize>().unwrap()),
            Some(_) => Val::Reg(value.chars().nth(0).unwrap()),
            None => panic!("Malformed input"),
        }
    }

    fn value(&self, vproc: &Proc) -> isize {
        match *self {
            Val::Int(v) => v,
            Val::Reg(v) => *vproc.regs.get(&v).unwrap_or(&0),
        }
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Snd(Val),
    Set(Val, Val),
    Add(Val, Val),
    Mul(Val, Val),
    Mod(Val, Val),
    Rcv(Val),
    Jgz(Val, Val),
}

impl Instruction {
    fn new(value: &str) -> Instruction {
        match value.chars().take(3).collect::<String>().as_ref() {
            p@"snd" | p@"rcv" => {
                let val = Val::new(value.chars().skip(4).collect::<String>().trim().as_ref());
                match p {
                    "snd" => Instruction::Snd(val),
                    "rcv" => Instruction::Rcv(val),
                    _ => panic!("Malformed input"),
                }
            },
            p@"set" | p@"add" | p@"mul" | p@"mod" | p@"jgz" => {
                let rest = value.chars().skip(4).collect::<String>();
                let parts: Vec<&str> = rest.trim().split(" ").collect();
                let val_a = Val::new(parts[0]);
                let val_b = Val::new(parts[1]);
                match p {
                    "set" => Instruction::Set(val_a, val_b),
                    "add" => Instruction::Add(val_a, val_b),
                    "mul" => Instruction::Mul(val_a, val_b),
                    "mod" => Instruction::Mod(val_a, val_b),
                    "jgz" => Instruction::Jgz(val_a, val_b),
                    _ => panic!("Malformed input"),
                }
            },
            p@_ => panic!("Malformed input {}", p)
        }
    }

    fn execute(self, vproc: &mut Proc) -> bool {
        match self {
            Instruction::Snd(a) => {
                vproc.last = Some(a.value(vproc));
                true
            },
            Instruction::Set(a, b) => {
                if let Val::Reg(reg) = a {
                    let val = b.value(vproc);
                    vproc.regs.insert(reg, val);
                }
                true
            },
            Instruction::Add(a, b) => {
                if let Val::Reg(reg) = a {
                    let val = a.value(vproc) + b.value(vproc);
                    vproc.regs.insert(reg, val);
                }
                true
            },
            Instruction::Mul(a, b) => {
                if let Val::Reg(reg) = a {
                    let val = a.value(vproc) * b.value(vproc);
                    vproc.regs.insert(reg, val);
                }
                true
            },
            Instruction::Mod(a, b) => {
                if let Val::Reg(reg) = a {
                    let val = a.value(vproc) % b.value(vproc);
                    vproc.regs.insert(reg, val);
                }
                true
            },
            Instruction::Rcv(a) => {
                if a.value(vproc) != 0 { return false }
                true
            },
            Instruction::Jgz(a, b) => {
                if a.value(vproc) <= 0 { return true }
                let new_pc = vproc.pc as isize + b.value(vproc);
                vproc.pc = new_pc as usize - 1;
                new_pc >= 0 || new_pc < vproc.instructions.len() as isize
            },
        }
    }
}

#[derive(Debug, Clone)]
struct Proc {
    instructions: Program,
    pc: usize,
    regs: HashMap<char, isize>,
    last: Option<isize>,
}

impl Proc {
    fn new(instructions: Program) -> Proc {
        Proc {
            instructions,
            pc: 0,
            regs: HashMap::new(),
            last: None
        }
    }

    fn run(&mut self) -> bool {
        if self.pc >= self.instructions.len() { return false }
        let instruction = self.instructions[self.pc as usize].clone();
        self.pc = self.pc + 1;
        let v = instruction.execute(self);
        v
    }
}

/// Solve the eighteenth problem.
pub fn solve() {
    // Open the input file of the problem
    let mut file = match File::open("input/18.txt") {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Error: Input file for the problem 18 not found");
            process::exit(1)
        }
    };
    // Read the file in a String variable
    let mut content: String = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => (),
        Err(_) => {
            eprintln!("Error: Input file for the problem 18 can't be read");
            process::exit(1)
        }
    };
    let instructions = content
        .lines()
        .map(|line| Instruction::new(line.trim()))
        .collect();
    // Compute and print the solutions of the two parts
    println!("18. Solutions to the eighteenth problem:");
    println!("\tFirst part: {}", solve_first_part(&instructions));
}

fn solve_first_part(instructions: &Vec<Instruction>) -> isize {
    let mut vproc = Proc::new((*instructions).clone());
    while vproc.run() {}
    vproc.last.unwrap()
}

#[cfg(test)]
mod tests {

    use super::{Instruction, solve_first_part};

    const CONTENT: &str = "set a 1
                           add a 2
                           mul a a
                           mod a 5
                           snd a
                           set a 0
                           rcv a
                           jgz a -1
                           set a 1
                           jgz a -2";

    #[test]
    fn eighteenth_problem_first_part() {
        let instructions = CONTENT.lines()
                                  .map(|line| Instruction::new(line.trim()))
                                  .collect();
        assert_eq!(solve_first_part(&instructions), 4);
    }

}
