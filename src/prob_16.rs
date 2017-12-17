use std::fs::File;
use std::io::Read;
use std::ops::{Index, IndexMut};
use std::process;

const NB_PROGS: usize = 16;

#[derive(Debug)]
struct Buffer {
    progs: [char; NB_PROGS],
    shift: usize,
}

impl Index<usize> for Buffer {
    type Output = char;
    fn index<'a>(&'a self, index: usize) -> &'a char {
        &self.progs[(self.shift + index) % NB_PROGS]
    }
}

impl IndexMut<usize> for Buffer {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut char {
        &mut self.progs[(self.shift + index) % NB_PROGS]
    }
}

impl Buffer {
    fn new() -> Buffer {
        Buffer { progs: progs(), shift: 0 }
    }

    fn shift(&mut self, size: usize) {
        self.shift = (self.shift + (NB_PROGS - size)) % NB_PROGS;
    }

    fn swap(&mut self, a: usize, b: usize) {
        let temp = self[a];
        self[a] = self[b];
        self[b] = temp;
    }

    fn swap_val(&mut self, a: char, b: char) {
        let (a, _) = self.progs.iter().enumerate().find(|&(_, v)| *v == a).unwrap();
        let (b, _) = self.progs.iter().enumerate().find(|&(_, v)| *v == b).unwrap();
        let temp = self.progs[a];
        self.progs[a] = self.progs[b];
        self.progs[b] = temp;
    }
    
    fn content(&mut self) -> String {
        let mut result: String = String::new();
        for i in 0..NB_PROGS {
            result.push(self[i])
        }
        result
    }
}

#[derive(Debug)]
enum Operation {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl Operation {
    fn new(val: &str) -> Operation {
        let mut iter = val.chars();
        match iter.next().unwrap() {
            's' => {
                let num: usize = iter.collect::<String>().parse().unwrap();   
                Operation::Spin(num)
            },
            'x' => {
                let rest: String = iter.collect();
                let parts: Vec<&str> = rest.split("/").collect();
                Operation::Exchange(parts[0].parse().unwrap(), parts[1].parse().unwrap())
            },
            'p' => {
                let rest: String = iter.collect();
                let parts: Vec<&str> = rest.split("/").collect();
                Operation::Partner(parts[0].chars().nth(0).unwrap(), parts[1].chars().nth(0).unwrap())
            },
            _ => panic!("Malformed input")
        }
    }
}

/// Solve the sixteenth problem.
pub fn solve() {
    // Open the input file of the problem
    let mut file = match File::open("input/16.txt") {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Error: Input file for the problem 16 not found");
            process::exit(1)
        }
    };
    // Read the file in a String variable
    let mut content: String = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => (),
        Err(_) => {
            eprintln!("Error: Input file for the problem 16 can't be read");
            process::exit(1)
        }
    };
    let operations: Vec<Operation> = content.split(",")
                                            .map(|v| Operation::new(v))
                                            .collect();
    // Compute and print the solutions of the two parts
    println!("16. Solutions to the sixteenth problem:");
    println!("\tFirst part: {}", solve_first_part(&operations));
    println!("\tSecond part: {}", solve_second_part(&operations));
}

fn progs() -> [char; NB_PROGS] {
    let mut progs: [char; NB_PROGS] = ['a'; NB_PROGS];
    for i in 0..NB_PROGS {
        progs[i] = (97 + i as u8) as char;
    }
    progs
}

fn apply(buffer: &mut Buffer, operation: &Operation) {
    match *operation {
        Operation::Spin(size) => buffer.shift(size),
        Operation::Exchange(a, b) => buffer.swap(a, b),
        Operation::Partner(a, b) => buffer.swap_val(a, b)
    }
}

fn solve_generic(operations: &Vec<Operation>, cycles: u64) -> String {
    let mut buffer = Buffer::new();
    for cycle in 0..cycles {
        if cycle % 1_000_000 == 0 { println!("{}", cycle) }
        for operation in operations {
            apply(&mut buffer, operation);
        }
    }
    buffer.content()
}

fn solve_first_part(operations: &Vec<Operation>) -> String {
    solve_generic(operations, 1)
}

fn solve_second_part(operations: &Vec<Operation>) -> String {
    // They choose 1_000_000_000 cycles to force us to not apply the brut force.
    // I'm not following rules B-)
    solve_generic(operations, 1_000_000_000)
}

#[cfg(test)]
mod tests {

    use super::{Operation, solve_first_part};

    #[test]
    fn sixteenth_problem_first_part() {
        let content = "s2,x3/4,pe/b";
        let operations: Vec<Operation> = content.split(",")
                                                .map(|v| Operation::new(v))
                                                .collect();
        assert_eq!(solve_first_part(&operations), "baedc");
    }

}
