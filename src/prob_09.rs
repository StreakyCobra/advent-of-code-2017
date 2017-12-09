use std::fs::File;
use std::io::Read;
use std::process;

#[derive(Debug)]
enum Group {
    Group,
    Garbage,
}

/// Solve the nineth problem.
pub fn solve() {
    // Open the input file of the problem
    let mut file = match File::open("input/09.txt") {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Error: Input file for the problem 09 not found");
            process::exit(1)
        }
    };
    // Read the file in a String variable
    let mut content: String = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => (),
        Err(_) => {
            eprintln!("Error: Input file for the problem 09 can't be read");
            process::exit(1)
        }
    };
    // Compute and print the solutions of the two parts
    println!("9. Solutions to the nineth problem:");
    println!("\tFirst part: {}", solve_first_part(&content));
    println!("\tSecond part: {}", solve_second_part(&content));
}

fn count_groups(input: &str, group: Group, level: u32) -> (u32, u32) {
    // Recursivity termination
    if input.chars().count() == 0 {
        return (0, 0)
    }
    // Get current character
    let current: char = input.chars().nth(0).unwrap();
    // If it's an ignore, skip the next character
    if let '!' = current {
        return count_groups(&input[2..], group, level)
    }
    // Create variables
    let mut val: u32 = 0;
    let mut gbc: u32 = 0;
    let new_group: Group;
    let new_level: u32;
    // Match behaviour according to the group
    match group {
        Group::Group => {
            match current {
                '{' => {
                    val = level;
                    new_group = Group::Group;
                    new_level = level + 1;
                },
                '}' => {
                    new_group = Group::Group;
                    new_level = level - 1;
                },
                '<' => {
                    new_group = Group::Garbage;
                    new_level = level;
                },
                _ => {
                    new_group = group;
                    new_level = level;
                },
            }
        },
        Group::Garbage => {
            match current {
                '<' => {
                    gbc = 1;
                    new_group = Group::Garbage;
                    new_level = level;
                },
                '>' => {
                    new_group = Group::Group;
                    new_level = level;
                },
                _ => {
                    gbc = 1;
                    new_group = group;
                    new_level = level;
                },
            }
        },
    }
    let rec = count_groups(&input[1..], new_group, new_level);
    return (rec.0 + val, rec.1 + gbc)
}

fn solve_first_part(input: &str) -> u32 {
    count_groups(input, Group::Group, 1).0
}

fn solve_second_part(input: &str) -> u32 {
    count_groups(input, Group::Group, 1).1
}

#[cfg(test)]
mod tests {

    use super::{solve_first_part, solve_second_part};

    #[test]
    fn nineth_problem_first_part() {
        assert_eq!(solve_first_part("{}"), 1);
        assert_eq!(solve_first_part("{{{}}}"), 6);
        assert_eq!(solve_first_part("{{},{}}"), 5);
        assert_eq!(solve_first_part("{{{},{},{{}}}}"), 16);
        assert_eq!(solve_first_part("{<a>,<a>,<a>,<a>}"), 1);
        assert_eq!(solve_first_part("{{<ab>},{<ab>},{<ab>},{<ab>}}"), 9);
        assert_eq!(solve_first_part("{{<!!>},{<!!>},{<!!>},{<!!>}}"), 9);
        assert_eq!(solve_first_part("{{<a!>},{<a!>},{<a!>},{<ab>}}"), 3);
    }

    #[test]
    fn nineth_problem_second_part() {
        assert_eq!(solve_second_part("{<>}"), 0);
        assert_eq!(solve_second_part("{<random characters>}"), 17);
        assert_eq!(solve_second_part("{<<<<>}"), 3);
        assert_eq!(solve_second_part("{<{!>}>}"), 2);
        assert_eq!(solve_second_part("{<!!>}"), 0);
        assert_eq!(solve_second_part("{<!!!>>}"), 0);
        assert_eq!(solve_second_part("{<{o\"i!a,<{i<a>}"), 10);
    }

}
