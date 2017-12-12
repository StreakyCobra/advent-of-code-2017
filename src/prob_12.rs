use std::fs::File;
use std::io::Read;
use std::process;
use std::collections::{HashMap, HashSet};

/// Solve the twelfth problem.
pub fn solve() {
    // Open the input file of the problem
    let mut file = match File::open("input/12.txt") {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Error: Input file for the problem 12 not found");
            process::exit(1)
        }
    };
    // Read the file in a String variable
    let mut content: String = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => (),
        Err(_) => {
            eprintln!("Error: Input file for the problem 12 can't be read");
            process::exit(1)
        }
    };
    let pipes = parse(&content);
    // Compute and print the solutions of the two parts
    println!("12. Solutions to the twelfth problem:");
    println!("\tFirst part: {}", solve_first_part(&pipes));
    println!("\tSecond part: {}", solve_second_part(&pipes));
}

fn parse(content: &str) -> HashMap<String, Vec<String>> {
    content.lines()
           .map(|line| {
               let mut it = line.trim().split_whitespace();
               let from = String::from(it.next().unwrap());
               it.next();  // Skip arrow <->
               let rest: String = it.collect();
               let to: Vec<String> = rest.split(",").map(|v| String::from(v)).collect();
               (from, to)
           })
           .collect()
}

fn get_group(val: &str, pipes: &HashMap<String, Vec<String>>) -> HashSet<String> {
    let mut group: HashMap<String, Vec<String>> = HashMap::new();
    let mut buffer: Vec<String> = vec![String::from(val)];
    while buffer.len() > 0 {
        let key = buffer.pop().unwrap();
        if group.contains_key(&key) { continue }
        let connected = pipes.get(&key).unwrap().clone();
        for conn in connected.clone() {
            if !group.contains_key(&conn) {
                buffer.push(conn.clone())
            }
        }
        group.insert(key.clone(), connected);
    }
    let mut set: HashSet<String> = HashSet::new();
    for (k, v) in group {
        set.insert(k.clone());
        set.extend(v);
    }
    set
}

fn solve_first_part(pipes: &HashMap<String, Vec<String>>) -> usize {
    get_group("0", &pipes).len()
}

fn solve_second_part(pipes: &HashMap<String, Vec<String>>) -> usize {
    let mut all: HashSet<String> = HashSet::new();
    for (k, v) in pipes {
        all.insert(k.clone());
        all.extend(v.clone());
    }
    let initial = get_group("0", pipes);
    let mut rest: HashSet<String> = all.difference(&initial).map(|v| v.clone()).collect();
    let mut groups: Vec<String> = vec![String::from("0")];
    while rest.len() > 0 {
        let rest_clone = rest.clone();
        let mut iter = rest_clone.iter();
        let head = iter.next().unwrap();
        let tail: HashSet<String> = iter.map(|v| v.clone()).collect();
        groups.push(head.clone());
        let group = get_group(&head, pipes);
        rest = tail.difference(&group).map(|v| v.clone()).collect();
    }
    groups.len()
}

#[cfg(test)]
mod tests {

    use super::{parse, solve_first_part, solve_second_part};

    const INPUT: &str = "0 <-> 2
                         1 <-> 1
                         2 <-> 0, 3, 4
                         3 <-> 2, 4
                         4 <-> 2, 3, 6
                         5 <-> 6
                         6 <-> 4, 5";

    #[test]
    fn twelfth_problem_first_part() {
        assert_eq!(solve_first_part(&parse(INPUT)), 6);
    }

    #[test]
    fn twelfth_problem_second_part() {
        assert_eq!(solve_second_part(&parse(INPUT)), 2);
    }

}
