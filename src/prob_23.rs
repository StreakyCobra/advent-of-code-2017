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
    Set(Val, Val),
    Sub(Val, Val),
    Mul(Val, Val),
    Jnz(Val, Val),
}

impl Instruction {
    fn new(value: &str) -> Instruction {
        match value.chars().take(3).collect::<String>().as_ref() {
            p@"set" | p@"sub" | p@"mul" | p@"jnz" => {
                let rest = value.chars().skip(4).collect::<String>();
                let parts: Vec<&str> = rest.trim().split(" ").collect();
                let val_a = Val::new(parts[0]);
                let val_b = Val::new(parts[1]);
                match p {
                    "set" => Instruction::Set(val_a, val_b),
                    "sub" => Instruction::Sub(val_a, val_b),
                    "mul" => Instruction::Mul(val_a, val_b),
                    "jnz" => Instruction::Jnz(val_a, val_b),
                    _ => panic!("Malformed input"),
                }
            },
            p@_ => panic!("Malformed input {}", p)
        }
    }

    fn execute(self, vproc: &mut Proc) -> bool {
        match self {
            Instruction::Set(a, b) => {
                if let Val::Reg(reg) = a {
                    let val = b.value(vproc);
                    vproc.regs.insert(reg, val);
                }
                true
            },
            Instruction::Sub(a, b) => {
                if let Val::Reg(reg) = a {
                    let val = a.value(vproc) - b.value(vproc);
                    vproc.regs.insert(reg, val);
                }
                true
            },
            Instruction::Mul(a, b) => {
                if let Val::Reg(reg) = a {
                    let val = a.value(vproc) * b.value(vproc);
                    vproc.regs.insert(reg, val);
                    vproc.result += 1;
                }
                true
            },
            Instruction::Jnz(a, b) => {
                if a.value(vproc) == 0 { return true }
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
    result: isize,
    queue: VecDeque<isize>,
    snd_count: usize,
    is_waiting: bool,
}

impl Proc {
    fn new(instructions: Program) -> Proc {
        Proc {
            instructions,
            pc: 0,
            regs: HashMap::new(),
            result: 0,
            queue: VecDeque::new(),
            snd_count: 0,
            is_waiting: false,
        }
    }

    fn run(&mut self) -> bool {
        if self.pc >= self.instructions.len() { return false }
        let instruction = self.instructions[self.pc as usize].clone();
        self.pc = self.pc + 1;
        // println!("{:?}", &self.pc);
        // println!("{:?}", &instb);
        // println!("{:?}", &self.regs);
        // println!("\n\n\n");
        let v = instruction.execute(self);
        v
    }
}

/// Solve the twenty-third problem.
pub fn solve() {
    // Open the input file of the problem
    let mut file = match File::open("input/23.txt") {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Error: Input file for the problem 23 not found");
            process::exit(1)
        }
    };
    // Read the file in a String variable
    let mut content: String = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => (),
        Err(_) => {
            eprintln!("Error: Input file for the problem 23 can't be read");
            process::exit(1)
        }
    };
    let instructions = content
        .lines()
        .map(|line| Instruction::new(line.trim()))
        .collect();
    // Compute and print the solutions of the second part
    println!("23. Solutions to the twenty-third problem:");
    println!("\tFirst part: {}", solve_first_part(&instructions));
    println!("\tSecond part: {}", solve_second_part(&instructions));
}

fn solve_first_part(instructions: &Vec<Instruction>) -> isize {
    let mut vproc = Proc::new((*instructions).clone());
    while vproc.run() {}
    vproc.result
}

fn solve_second_part(instructions: &Vec<Instruction>) -> isize {
    let mut vproc = Proc::new((*instructions).clone());
    vproc.regs.insert('a', 1);
    let mut count: usize = 0;
    while vproc.run() {
        count += 1;
        if count % 100_000_000 == 0 {
            println!("{:?}", &vproc.regs.get(&'h').unwrap_or(&0));
            println!("{:?}", &vproc.regs);
        }
    }
    *vproc.regs.get(&'h').unwrap()
}