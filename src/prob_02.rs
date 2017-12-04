use std::fs::File;
use std::io::Read;
use std::process;

use itertools::Itertools;


/// Solve the second problem.
pub fn solve() {
    // Open the input file of the problem
    let mut file = match File::open("input/02.txt") {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Error: Input file for the problem 02 not found");
            process::exit(1)
        }
    };
    // Read the file in a String variable
    let mut content: String = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => (),
        Err(_) => {
            eprintln!("Error: Input file for the problem 02 can't be read");
            process::exit(1)
        }
    };
    // Trim trailling spaces and EOL characters
    content = content.to_string();
    let matrix : Vec<Vec<u32>> = content.lines()
                                        .map(|line|
                                             line.split_whitespace()
                                                 .filter_map(|el| el.parse::<u32>().ok())
                                                 .collect())
                                        .collect();
    // Compute and print the solutions of the two parts
    println!("2. Solutions to the second problem:");
    println!("\tFirst part: {}", solve_first_part(&matrix));
    println!("\tSecond part: {}", solve_second_part(&matrix));
}

/// Solve the first part of the second problem.
fn solve_first_part(matrix: &Vec<Vec<u32>>) -> u32 {
    matrix.iter()
          .map(|line| line.iter().max().unwrap() - line.iter().min().unwrap())
          .sum()
}

/// Solve the second part of the second second problem.
fn solve_second_part(matrix: &Vec<Vec<u32>>) -> u32 {
    matrix.iter()
          .map(|line|
               line.iter()
                   .tuple_combinations()
                   .filter_map(|(x, y)|
                               if x % y == 0 {
                                   Some(x/y)
                               } else if y % x == 0 {
                                   Some(y/x)
                               } else {
                                   None
                               }
                               )
                   .sum::<u32>())
          .sum()
}

#[cfg(test)]
mod tests {

    use super::{solve_first_part, solve_second_part};

    #[test]
    fn test_cases_first_part() {
        assert_eq!(solve_first_part(&vec![vec![5, 1, 9, 5],
                                          vec![7, 5, 3],
                                          vec![2, 4, 6, 8]]), 18);
    }

    #[test]
    fn test_cases_second_part() {
        assert_eq!(solve_second_part(&vec![vec![5, 9, 2, 8],
                                           vec![9, 4, 7, 3],
                                           vec![3, 8, 6, 5]]), 9);
    }

}
