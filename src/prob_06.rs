use std::fs::File;
use std::io::Read;
use std::process;
use std::collections::HashMap;

/// Solve the sixth problem.
pub fn solve() {
    // Open the input file of the problem
    let mut file = match File::open("input/06.txt") {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Error: Input file for the problem 06 not found");
            process::exit(1)
        }
    };
    // Read the file in a String variable
    let mut content: String = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => (),
        Err(_) => {
            eprintln!("Error: Input file for the problem 06 can't be read");
            process::exit(1)
        }
    };
    let banks : Vec<isize> = content.split_whitespace()
                                    .map(|val| val.parse::<isize>().unwrap())
                                    .collect();
    // Compute and print the solutions of the two parts
    println!("6. Solutions to the sixth problem:");
    println!("\tFirst part: {}", solve_first_part(banks.clone()));
    println!("\tSecond part: {}", solve_second_part(banks.clone()));
}

fn solve_generic(mut banks: Vec<isize>) -> (usize, usize) {
    let mut step = 1usize;
    let mut seen: HashMap<Vec<isize>, usize> = HashMap::new();
    seen.insert(banks.clone(), 0);
    loop {
        // Find the biggest bank position and value
        let (p, c) = banks.clone()
                          .into_iter()
                          .enumerate()
                          .max_by_key(|&(i, v)| (v, 0-i as isize))
                          .unwrap();
        // Set it to zero
        banks[p] = 0;
        // Distribute its value to next banks
        for i in 1..c+1 {
            let j = (p + i as usize) % banks.len();
            banks[j] += 1;
        }
        // If the situation has already been seen, return the current step and the cycle
        if seen.contains_key(&banks) {
            return (step, step - seen[&banks])
        }
        // Othewise store the current situation as seen
        seen.insert(banks.clone(), step);
        // Increase the counter
        step += 1;
    }
}

fn solve_first_part(banks: Vec<isize>) -> usize {
    solve_generic(banks).0
}

fn solve_second_part(banks: Vec<isize>) -> usize {
    solve_generic(banks).1
}

#[cfg(test)]
mod tests {

    use super::{solve_first_part, solve_second_part};

    #[test]
    fn sixth_problem_first_part() {
        assert_eq!(solve_first_part(vec![0, 2, 7, 0]), 5);
    }

    #[test]
    fn sixth_problem_second_part() {
        assert_eq!(solve_second_part(vec![0, 2, 7, 0]), 4);
    }

}
