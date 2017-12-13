use std::fs::File;
use std::io::Read;
use std::process;
use std::collections::HashMap;

/// Solve the thirteenth problem.
pub fn solve() {
    // Open the input file of the problem
    let mut file = match File::open("input/13.txt") {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Error: Input file for the problem 13 not found");
            process::exit(1)
        }
    };
    // Read the file in a String variable
    let mut content: String = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => (),
        Err(_) => {
            eprintln!("Error: Input file for the problem 13 can't be read");
            process::exit(1)
        }
    };
    let layers = parse(&content);
    // Compute and print the solutions of the two parts
    println!("13. Solutions to the thirteenth problem:");
    println!("\tFirst part: {}", solve_first_part(&layers));
    println!("\tSecond part: {}", solve_second_part(&layers));
}

fn parse(content: &str) -> HashMap<usize, usize> {
    content.lines()
           .map(|line| {
               let mut components = line.split(": ");
               let layer: usize = components.next().unwrap().trim().parse().unwrap();
               let size: usize = components.next().unwrap().trim().parse().unwrap();
               (layer, size)
           })
           .collect()
}

fn score(delay: usize, layers: &HashMap<usize, usize>) -> (usize, Vec<usize>) {
    let cycles: Vec<usize> = layers.iter()
                                .map(|(l, s)| {
                                    let cycle = (*s-1) * 2;
                                    let mut seconds: Vec<usize> = Vec::new();
                                    for i in delay..(*l+delay+1) {
                                        if i % cycle == 0 {
                                            seconds.push(i);
                                        }
                                    }
                                    (*l, seconds)
                                })
                                .filter(|&(l, ref s)| s.contains(&(l + delay)))
                                .map(|(l, _)| l)
                                .collect();
    let score = layers.iter()
                      .filter(|&(l, _)| cycles.contains(l))
                      .map(|(l, s)| l*s)
                      .sum();
    (score, cycles.clone())
}

fn solve_first_part(layers: &HashMap<usize, usize>) -> usize {
    score(0, &layers).0
}

fn solve_second_part(layers: &HashMap<usize, usize>) -> usize {
    let mut i: usize = 0;
    loop {
        let (_, cycles) = score(i, &layers);
        if cycles.len() == 0 { return i }
        i += 1;
    }
}

#[cfg(test)]
mod tests {

    use super::{parse, solve_first_part, solve_second_part};

    const INPUT: &str = "0: 3
                         1: 2
                         4: 4
                         6: 4";

    #[test]
    fn thirteenth_problem_first_part() {
        assert_eq!(solve_first_part(&parse(INPUT)), 24);
    }

    #[test]
    fn thirteenth_problem_second_part() {
        assert_eq!(solve_second_part(&parse(INPUT)), 10);
    }

}
