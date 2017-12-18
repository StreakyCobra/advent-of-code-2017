use std::fs::File;
use std::io::Read;
use std::process;
use std::collections::{HashMap, VecDeque};

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

    fn execute(self, vproc: &mut Proc, vproc_other: &mut Proc) -> bool {
        match self {
            Instruction::Snd(a) => {
                vproc_other.queue.push_back(a.value(vproc));
                vproc.snd_count += 1;
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
                if vproc.queue.len() == 0 {
                    vproc.is_waiting = true;
                    vproc.pc -= 1;
                    return true
                }
                if let Val::Reg(a) = a {
                    vproc.is_waiting = false;
                    vproc.regs.insert(a, vproc.queue.pop_front().unwrap());
                }
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
    name: usize,
    instructions: Program,
    pc: usize,
    regs: HashMap<char, isize>,
    last: Option<isize>,
    queue: VecDeque<isize>,
    snd_count: usize,
    is_waiting: bool,
}

impl Proc {
    fn new(name: usize, instructions: Program) -> Proc {
        Proc {
            name,
            instructions,
            pc: 0,
            regs: HashMap::new(),
            last: None,
            queue: VecDeque::new(),
            snd_count: 0,
            is_waiting: false,
        }
    }

    fn run(&mut self, other: &mut Proc) -> bool {
        if self.pc >= self.instructions.len() { return false }
        let instruction = self.instructions[self.pc as usize].clone();
        self.pc = self.pc + 1;
        let v = instruction.execute(self, other);
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
    // Compute and print the solutions of the second part
    println!("\tSecond part: {}", solve_second_part(&instructions));
}


fn solve_second_part(instructions: &Vec<Instruction>) -> usize {
    let mut vproc0 = Proc::new(0, (*instructions).clone());
    let mut vproc1 = Proc::new(1, (*instructions).clone());
    vproc0.regs.insert('p', 0);
    vproc1.regs.insert('p', 1);
    let mut v0_finish = false;
    let mut v1_finish = false;
    loop {
        if !v0_finish && !vproc0.run(&mut vproc1) { v0_finish = true }
        if !v1_finish && !vproc1.run(&mut vproc0) { v1_finish = true }
        if v0_finish && v1_finish { break }
        if vproc0.is_waiting && vproc1.is_waiting {break}
    }
    vproc1.snd_count
}

#[cfg(test)]
mod tests {

    use super::{Instruction, solve_second_part};

    const CONTENT: &str = "snd 1
                           snd 2
                           snd p
                           rcv a
                           rcv b
                           rcv c
                           rcv d";

    #[test]
    fn eighteenth_problem_second_part() {
        let instructions = CONTENT.lines()
                                  .map(|line| Instruction::new(line.trim()))
                                  .collect();
        assert_eq!(solve_second_part(&instructions), 3);
    }

}
