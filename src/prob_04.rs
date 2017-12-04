use std::fs::File;
use std::io::Read;
use std::process;

/// Solve the fourth problem.
pub fn solve() {
    // Open the input file of the problem
    let mut file = match File::open("input/04.txt") {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Error: Input file for the problem 04 not found");
            process::exit(1)
        }
    };
    // Read the file in a String variable
    let mut content: String = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => (),
        Err(_) => {
            eprintln!("Error: Input file for the problem 04 can't be read");
            process::exit(1)
        }
    };
    let passwords = content.lines().collect();
    // Compute and print the solutions of the two parts
    println!("4. Solutions to the fourth problem:");
    println!("\tFirst part: {}", solve_first_part(&passwords));
    println!("\tSecond part: {}", solve_second_part(&passwords));
}

/// Solve the first part of the fourth problem.
fn solve_first_part(passwords : &Vec<&str>) -> u32 {
    passwords.iter()
             .filter_map(|line| {
                  let words : Vec<&str> = line.split_whitespace().collect();
                  let mut uniq = words.clone();
                  uniq.sort();
                  uniq.dedup();
                  if words.len() == uniq.len() {
                      Some(*line)
                  } else {
                      None
                  }
             })
             .count() as u32
}

/// Solve the second part of the fourth problem.
fn solve_second_part(passwords : &Vec<&str>) -> u32 {
    passwords.iter()
             .filter_map(|line| {
                  let words : Vec<String> = line.split_whitespace()
                                                .map(|word| {
                                                    let mut chars : Vec<char> = word.chars().collect();
                                                    chars.sort();
                                                    chars.iter().collect()
                                                })
                                                .collect();
                  let mut uniq = words.clone();
                  uniq.sort();
                  uniq.dedup();
                  if words.len() == uniq.len() {
                      Some(*line)
                  } else {
                      None
                  }
             })
             .count() as u32
}

#[cfg(test)]
mod tests {

    use super::{solve_first_part, solve_second_part};

    #[test]
    fn fourth_problem_first_part() {
        assert_eq!(solve_first_part(&vec!["aa bb cc dd ee",
                                          "aa bb cc dd aa",
                                          "aa bb cc dd aaa"]), 2);
    }

    #[test]
    fn fourth_problem_second_part() {
        assert_eq!(solve_second_part(&vec!["abcde fghij",
                                           "abcde xyz ecdab",
                                           "a ab abc abd abf abj",
                                           "iiii oiii ooii oooi oooo",
                                           "oiii ioii iioi iiio"]), 3);
    }

}
