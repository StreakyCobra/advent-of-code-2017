use std::fs::File;
use std::io::Read;
use std::process;
use std::collections::HashMap;

type Grid = HashMap<(isize, isize), State>;

#[derive(Debug)]
enum State {
    Weakened,
    Infected,
    Flagged,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn next(&self, x: isize, y: isize) -> (isize, isize) {
        let mut new_x = x;
        let mut new_y = y;
        match *self {
            Direction::Up => new_x -= 1,
            Direction::Down => new_x += 1,
            Direction::Left => new_y -= 1,
            Direction::Right => new_y += 1,
        };
        (new_x, new_y)
    }

    fn left(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    fn right(&self) -> Direction {
        self.left().left().left()
    }
}

/// Solve the twenty-second problem.
pub fn solve() {
    // Open the input file of the problem
    let mut file = match File::open("input/22.txt") {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Error: Input file for the problem 22 not found");
            process::exit(1)
        }
    };
    // Read the file in a String variable
    let mut content: String = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => (),
        Err(_) => {
            eprintln!("Error: Input file for the problem 22 can't be read");
            process::exit(1)
        }
    };
    // Compute and print the solutions of the first part
    println!("\tSecond part: {}", solve_second_part(parse(&content), 10000000));
}

fn parse(input: &str) -> Grid {
    let mut grid: Grid = HashMap::new();
    let center: Vec<Vec<bool>> = 
        input.lines()
             .map(|line| line.chars().map(|c| c == '#').collect())
             .collect();
    let size = center.len();
    let shift = (size / 2) as isize;
    for i in 0..size {
        for j in 0..size {
            if center[i][j] {
                grid.insert((i as isize - shift,
                             j as isize - shift), State::Infected);
            }
        }
    }
    grid
}

fn solve_second_part(mut grid: Grid, cycles: usize) -> usize {
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut dir: Direction = Direction::Up;
    let mut count: usize = 0;
    for _ in 0..cycles {
        match grid.contains_key(&(x,y)) {
            false => {
                grid.insert((x, y), State::Weakened);
                dir = dir.left();
            },
            true => {
                match *grid.get(&(x,y)).unwrap() {
                    State::Weakened => {
                        grid.insert((x, y), State::Infected);
                        count += 1;
                    },
                    State::Infected => {
                        grid.insert((x, y), State::Flagged);
                        dir = dir.right();
                    },
                    State::Flagged => {
                        grid.remove(&(x, y));
                        dir = dir.left().left();
                    },
                }
            },
        }
        let (next_x, next_y) = dir.next(x, y);
        x = next_x;
        y = next_y;
    }
    count
}

#[cfg(test)]
mod tests {

    use super::{parse, solve_second_part};

    const INPUT: &str = "..#\n#..\n...";

    #[test]
    fn twentysecond_problem_second_part() {
        assert_eq!(solve_second_part(parse(INPUT), 100), 26);
        assert_eq!(solve_second_part(parse(INPUT), 10_000_000), 2511944);
    }

}
