use std::fs::File;
use std::io::Read;
use std::process;


/// Solve the first problem.
pub fn solve() {
    // Open the input file of the problem
    let mut file = match File::open("input/01.txt") {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Error: Input file for the problem 01 not found");
            process::exit(1)
        }
    };
    // Read the file in a String variable
    let mut digits: String = String::new();
    match file.read_to_string(&mut digits) {
        Ok(_) => (),
        Err(_) => {
            eprintln!("Error: Input file for the problem 01 can't be read");
            process::exit(1)
        }
    };
    // Trim trailling spaces and EOL characters
    digits = digits.trim().to_string();
    // Compute and print the solutions of the two parts
    println!("1. Solutions to the first problem:");
    println!("\tFirst part: {}", solve_first_part(&digits));
    println!("\tSecond part: {}", solve_second_part(&digits));
}

/// Solve the first part of the first problem.
fn solve_first_part(digits: &String) -> u32 {
    // The digit to compare is the next one (module length)
    let func = |i| i + 1;
    // Solve the generic problem with the part-specific function
    solve_generic_part(digits, &func)
}

/// Solve the second part of the first problem.
fn solve_second_part(digits: &String) -> u32 {
    // The digit to compare is the one halfway further (module length)
    let func = |i| i + digits.len() / 2;
    // Solve the generic problem with the part-specific function
    solve_generic_part(digits, &func)
}

/// Generic solver for the two parts of the first problem
fn solve_generic_part(digits: &String, func: &Fn(usize) -> usize) -> u32 {
    // Get the number of digits
    let length = digits.len();
    // Get the digits as a chars array
    let chars = digits.chars();
    // Initialize the sum to zero
    let mut sum = 0;
    // Iterate over all digits
    for (i, d) in chars.clone().enumerate() {
        // Compute the position of the related digit using the given function
        let pos = func(i) % length;
        // Get the related digit
        let x = chars.clone().nth(pos).unwrap();
        // If the digits are the same, add the value to the sum
        if d == x { sum += x.to_digit(10).unwrap() }
    }
    // Return the solution
    return sum
}
