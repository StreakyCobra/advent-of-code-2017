use std::fs::File;
use std::io::Read;
use std::process;

type Maze = Vec<Vec<char>>;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn next(&self, x: usize, y: usize) -> (usize, usize) {
        let mut new_x = x;
        let mut new_y = y;
        match *self {
            Direction::Up => new_y -= 1,
            Direction::Down => new_y += 1,
            Direction::Left => new_x -= 1,
            Direction::Right => new_x += 1,
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

/// Solve the nineteenth problem.
pub fn solve() {
    // Open the input file of the problem
    let mut file = match File::open("input/19.txt") {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Error: Input file for the problem 19 not found");
            process::exit(1)
        }
    };
    // Read the file in a String variable
    let mut content: String = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => (),
        Err(_) => {
            eprintln!("Error: Input file for the problem 19 can't be read");
            process::exit(1)
        }
    };
    // Compute and print the solutions of the two parts
    println!("19. Solutions to the nineteenth problem:");
    println!("\tFirst part: {}", solve_first_part(parse(&content)));
    println!("\tSecond part: {}", solve_second_part(parse(&content)));
}

fn parse(input: &str) -> Maze {
    input.lines()
         .map(|line| line.chars().collect())
         .collect()
}

fn solve_generic(maze: Maze) -> (String, usize) {
    let mut x: usize = maze[0].iter().enumerate().find(|&(_, c)| *c == '|').map(|(i, _)| i).unwrap();
    let mut y: usize = 0;
    let mut dir: Direction = Direction::Down;
    let mut txt: String = String::new();
    let mut count: usize = 0;
    loop {
        count += 1;
        let (next_x, next_y) = dir.next(x, y);
        x = next_x;
        y = next_y;
        let next = maze[y][x];
        match next {
            ' ' => break,
            'A'...'Z' => txt.push(next),
            '+' => {
                let (left_x, left_y) = dir.left().next(x, y);
                if maze[left_y][left_x] != ' ' {
                    dir = dir.left();
                } else {
                    dir = dir.right();
                }
            },
            _ => ()
        }
    }
    (txt, count)
}

fn solve_first_part(maze: Maze) -> String {
    solve_generic(maze).0
}

fn solve_second_part(maze: Maze) -> usize {
    solve_generic(maze).1
}

#[cfg(test)]
mod tests {

    use super::{parse, solve_first_part, solve_second_part};

    const INPUT: &str = "     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ 
     ";


    #[test]
    fn nineteenth_problem_first_part() {
        assert_eq!(solve_first_part(parse(INPUT)), "ABCDEF");
    }

    #[test]
    fn nineteenth_problem_second_part() {
        assert_eq!(solve_second_part(parse(INPUT)), 38);
    }

}
