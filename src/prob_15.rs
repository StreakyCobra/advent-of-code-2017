use std::fs::File;
use std::io::Read;
use std::process;

const A_FACTOR: u64 = 16807;
const B_FACTOR: u64 = 48271;
const MODULO: u64 = 2147483647;

struct Generator {
    current: u64,
    factor: u64,
    modulo: u64,
}

impl Generator {
    fn new(base: u64, factor: u64, modulo: u64) -> Generator {
        Generator {
            current: base,
            factor,
            modulo,
        }
    }
}

impl Iterator for Generator {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        self.current = (self.current * self.factor) % self.modulo;
        Some(self.current)
    }
}

/// Solve the fifteenth problem.
pub fn solve() {
    // Open the input file of the problem
    let mut file = match File::open("input/15.txt") {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Error: Input file for the problem 15 not found");
            process::exit(1)
        }
    };
    // Read the file in a String variable
    let mut content: String = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => (),
        Err(_) => {
            eprintln!("Error: Input file for the problem 15 can't be read");
            process::exit(1)
        }
    };
    let mut iter = content.lines();
    let a_base: u64 = iter.next().unwrap()
                          .split_whitespace()
                          .nth(4).unwrap()
                          .parse().unwrap();
    let b_base: u64 = iter.next().unwrap()
                          .split_whitespace()
                          .nth(4).unwrap()
                          .parse().unwrap();
    // Compute and print the solutions of the two parts
    println!("15. Solutions to the fifteenth problem:");
    println!("\tFirst part: {}", solve_first_part(Generator::new(a_base, A_FACTOR, MODULO),
                                                  Generator::new(b_base, B_FACTOR, MODULO)));
    println!("\tSecond part: {}", solve_second_part(Generator::new(a_base, A_FACTOR, MODULO),
                                                    Generator::new(b_base, B_FACTOR, MODULO)));
}

fn solve_generic<I, L>(mut gen_a: I, mut gen_b: L, cycles: usize) -> u64
    where I: Iterator<Item = u64>,
          L: Iterator<Item = u64>
    {
    let mut count: u64 = 0;
    for _ in 0..cycles {
        let a = gen_a.next().unwrap();
        let b = gen_b.next().unwrap();
        if a & 0xFFFF == b & 0xFFFF { count += 1 }
    }
    count
}

fn solve_first_part(gen_a: Generator, gen_b: Generator) -> u64 {
    solve_generic(gen_a,
                  gen_b,
                  40_000_000)
}

fn solve_second_part(gen_a: Generator, gen_b: Generator) -> u64 {
    solve_generic(gen_a.filter(|v| v % 4 == 0),
                  gen_b.filter(|v| v % 8 == 0),
                  5_000_000)
}

#[cfg(test)]
mod tests {

    use super::Generator;
    use super::{A_FACTOR, B_FACTOR, MODULO};
    use super::{solve_first_part, solve_second_part};

    #[test]
    fn fifteenth_problem_first_part() {
        assert_eq!(solve_first_part(Generator::new(65, A_FACTOR, MODULO),
                                    Generator::new(8921, B_FACTOR, MODULO)),
                   588);
    }

    #[test]
    fn fifteenth_problem_second_part() {
        assert_eq!(solve_second_part(Generator::new(65, A_FACTOR, MODULO),
                                     Generator::new(8921, B_FACTOR, MODULO)),
                   309);
    }

}
