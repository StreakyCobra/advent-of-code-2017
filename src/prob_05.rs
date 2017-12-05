use std::fs::File;
use std::io::Read;
use std::process;

/// Solve the fifth problem.
pub fn solve() {
    // Open the input file of the problem
    let mut file = match File::open("input/05.txt") {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Error: Input file for the problem 05 not found");
            process::exit(1)
        }
    };
    // Read the file in a String variable
    let mut content: String = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => (),
        Err(_) => {
            eprintln!("Error: Input file for the problem 05 can't be read");
            process::exit(1)
        }
    };
    let maze : Vec<i32> = content.lines()
                                 .map(|line|
                                       line.parse::<i32>()
                                           .unwrap())
                                 .collect();
    // Compute and print the solutions of the two parts
    println!("5. Solutions to the fifth problem:");
    println!("\tFirst part: {}", solve_first_part(&mut maze.clone()));
    println!("\tSecond part: {}", solve_second_part(&mut maze.clone()));
}

fn solve_first_part(maze : &mut Vec<i32>) -> u32 {
    let mut count : u32 = 0;
    let mut pos : i32 = 0;
    loop {
        count += 1;
        let next = pos + maze[pos as usize];
        if next < 0 { break };
        if next as usize >= maze.len() { break };
        maze[pos as usize] += 1;
        pos = next;
    }
    count
}

fn solve_second_part(maze : &mut Vec<i32>) -> u32 {
    let mut count : u32 = 0;
    let mut pos : i32 = 0;
    loop {
        count += 1;
        let next = pos + maze[pos as usize];
        if next < 0 { break };
        if next as usize >= maze.len() { break };
        if maze[pos as usize] >= 3 {
            maze[pos as usize] -= 1;
        } else {
            maze[pos as usize] += 1;
        }
        pos = next;
    }
    count
}

#[cfg(test)]
mod tests {

    use super::{solve_first_part, solve_second_part};

    #[test]
    fn fifth_problem_first_part() {
        assert_eq!(solve_first_part(&mut vec![0, 3, 0, 1, -3]), 5);
    }

    #[test]
    fn fifth_problem_second_part() {
        assert_eq!(solve_second_part(&mut vec![0, 3, 0, 1, -3]), 10);
    }

}
