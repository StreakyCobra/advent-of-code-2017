use std::fs::File;
use std::process;
use std::io::Read;
use prob_10::solve_second_part as knot_hash;

/// Solve the fourteenth problem.
pub fn solve() {
    // Open the input file of the problem
    let mut file = match File::open("input/14.txt") {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Error: Input file for the problem 14 not found");
            process::exit(1)
        }
    };
    // Read the file in a String variable
    let mut content: String = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => (),
        Err(_) => {
            eprintln!("Error: Input file for the problem 14 can't be read");
            process::exit(1)
        }
    };
    let key = content.trim();
    // Compute and print the solutions of the two parts
    println!("14. Solutions to the fourteenth problem:");
    println!("\tFirst part: {}", solve_first_part(&key));
    println!("\tSecond part: {}", solve_second_part(&key));
}

fn solve_first_part(key: &str) -> usize {
    (0..128).map(|i| knot_hash(256, &format!("{}-{}", &key, i)))
            .map(|h| h.chars()
                      .map(|c| format!("{:04b}", c.to_digit(16).unwrap()))
                      .collect::<String>())
            .map(|b| b.chars()
                      .filter(|c| *c == '1')
                      .collect::<String>())
            .map(|h| h.len())
            .sum()
}

fn count_region(grid: &mut Vec<Vec<bool>>) -> usize {
    let mut score = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] {
                score += 1;
                merge(grid, i, j);
            }
        }
    }
    score
}

fn merge(grid: &mut Vec<Vec<bool>>, i: usize, j: usize) {
    if !grid[i][j] { return }

    grid[i][j] = false;

    if i > 0 {
        merge(grid, i-1, j);
    }
    if i < 127 {
        merge(grid, i+1, j);
    }
    if j > 0 {
        merge(grid, i, j-1);
    }
    if j < 127 {
        merge(grid, i, j+1);
    }
}

fn solve_second_part(key: &str) -> usize {
    let mut grid: Vec<Vec<bool>> = (0..128)
        .map(|i| knot_hash(256, &format!("{}-{}", &key, i)))
        .map(|h| h.chars()
                  .map(|c| format!("{:04b}", c.to_digit(16).unwrap()))
                  .collect::<String>())
        .map(|b| b.chars()
                  .map(|c| c == '1')
                  .collect())
        .collect();
    count_region(&mut grid)
}

#[cfg(test)]
mod tests {

    use super::{solve_first_part, solve_second_part};

    #[test]
    fn fourteenth_problem_first_part() {
        assert_eq!(solve_first_part("flqrgnkx"), 8108);
    }

    #[test]
    fn fourteenth_problem_second_part() {
        assert_eq!(solve_second_part("flqrgnkx"), 1242);
    }

}
