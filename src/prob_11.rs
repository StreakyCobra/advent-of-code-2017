use std::fs::File;
use std::io::Read;
use std::process;
use std::cmp;

struct Pos {
    x: isize,
    y: isize,
    z: isize,
}

impl Pos {
    fn mv(&mut self, direction: &Direction) {
        match direction {
            &Direction::N => {
                self.y += 1;
                self.z -= 1;
            },
            &Direction::NE => {
                self.x += 1;
                self.z -= 1;
            },
            &Direction::SE => {
                self.x += 1;
                self.y -= 1;
            },
            &Direction::S => {
                self.y -= 1;
                self.z += 1;
            },
            &Direction::SW => {
                self.x -= 1;
                self.z += 1;
            },
            &Direction::NW => {
                self.x -= 1;
                self.y += 1;
            },
        }
    }

    fn distance(&mut self) -> usize {
        let distance = (self.x.abs() + self.y.abs() + self.z.abs()) / 2;
        distance as usize
    }
}

enum Direction {
    N,
    NE,
    SE,
    S,
    SW,
    NW
}

impl<'a> From<&'a str> for Direction {
    fn from(value: &'a str) -> Direction {
        match value {
            "n" => Direction::N,
            "ne" => Direction::NE,
            "se" => Direction::SE,
            "s" => Direction::S,
            "sw" => Direction::SW,
            "nw" => Direction::NW,
            _ => panic!("String {} is not a valid direction", value),
        }
    }
}

/// Solve the eleventh problem.
pub fn solve() {
    // Open the input file of the problem
    let mut file = match File::open("input/11.txt") {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Error: Input file for the problem 11 not found");
            process::exit(1)
        }
    };
    // Read the file in a String variable
    let mut content: String = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => (),
        Err(_) => {
            eprintln!("Error: Input file for the problem 11 can't be read");
            process::exit(1)
        }
    };
    let directions: Vec<Direction> = content.trim()
                                            .split(",")
                                            .map(|v| Direction::from(v))
                                            .collect();
    // Compute and print the solutions of the two parts
    println!("11. Solutions to the eleventh problem:");
    println!("\tFirst part: {}", solve_first_part(&directions));
    println!("\tSecond part: {}", solve_second_part(&directions));
}

fn solve_first_part(directions: &Vec<Direction>) -> usize {
    let mut pos: Pos = Pos { x: 0, y: 0, z: 0 };
    directions.iter().for_each(|v| pos.mv(&v));
    pos.distance()
}

fn solve_second_part(directions: &Vec<Direction>) -> usize {
    let mut pos: Pos = Pos { x: 0, y: 0, z: 0 };
    let mut max_dist: usize = 0;
    directions.iter().for_each(|v| {
        pos.mv(&v);
        max_dist = cmp::max(max_dist, pos.distance());
        });
    max_dist
}

#[cfg(test)]
mod tests {

    use super::{Direction, solve_first_part, solve_second_part};

    #[test]
    fn eleventh_problem_first_part() {
        assert_eq!(solve_first_part(&vec![Direction::NE,
                                          Direction::NE,
                                          Direction::NE]), 3);
        assert_eq!(solve_first_part(&vec![Direction::NE,
                                          Direction::NE,
                                          Direction::SW,
                                          Direction::SW]), 0);
        assert_eq!(solve_first_part(&vec![Direction::NE,
                                          Direction::NE,
                                          Direction::S,
                                          Direction::S]), 2);
        assert_eq!(solve_first_part(&vec![Direction::SE,
                                          Direction::SW,
                                          Direction::SE,
                                          Direction::SW,
                                          Direction::SW]), 3);
    }

    #[test]
    fn eleventh_problem_second_part() {
        assert_eq!(solve_second_part(&vec![]), 4);
    }

}
