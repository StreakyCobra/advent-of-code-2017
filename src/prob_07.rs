use std::fs::File;
use std::io::Read;
use std::process;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;

/// Solve the seventh problem.
pub fn solve() {
    // Open the input file of the problem
    let mut file = match File::open("input/07.txt") {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Error: Input file for the problem 07 not found");
            process::exit(1)
        }
    };
    // Read the file in a String variable
    let mut content: String = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => (),
        Err(_) => {
            eprintln!("Error: Input file for the problem 07 can't be read");
            process::exit(1)
        }
    };
    // Compute and print the solutions of the two parts
    println!("7. Solutions to the seventh problem:");
    println!("\tFirst part: {}", solve_first_part(content.clone()));
    println!("\tSecond part: {}", solve_second_part(content.clone()));
}

fn parse(content: String) -> HashMap<String, (u32, Vec<String>)> {
    content.lines()
           .map(|line| {
               let mut parts = line.split_whitespace();
               let name = String::from(parts.next().unwrap());
               let weight = parts.next().unwrap();
               let weight = (&weight)[1..(weight.len()-1)].parse().unwrap();
               parts.next();
               let rest: String = parts.collect();
               let children: Vec<String> = rest.split(",")
                                               .map(|v| String::from(v))
                                               .filter(|v| !v.is_empty())
                                               .collect();
               (name, (weight, children))
            })
           .collect()
}

fn find_root(map: &HashMap<String, (u32, Vec<String>)>) -> String {
    let children: HashSet<String> = map.values()
                                       .map(|val| val.1.clone())
                                       .flatten()
                                       .collect();
    let keys: HashSet<String> = map.keys().map(|v| String::from(v.clone())).collect();
    keys.difference(&children).next().unwrap().clone()
}

fn solve_first_part(content: String) -> String {
    let map = parse(content);
    find_root(&map)
}

fn weight_rec(val: &String, map: &HashMap<String, (u32, Vec<String>)>, weights: &mut HashMap<String, u32>) -> u32 {
    match weights.clone().get(val) {
        Some(weight) => return *weight,
        None => {
            let mut total: u32 = 0;
            let current = map.get(val).unwrap();
            for c in current.1.iter() {
                let sum = weight_rec(c, map, weights);
                total += sum;
            }
            weights.insert(val.clone(), current.0 + total);
            return current.0 + total
        }
    }
}

fn real_weights(map: &HashMap<String, (u32, Vec<String>)>) -> HashMap<String, u32>{
    let mut weights: HashMap<String, u32> = HashMap::new();
    weight_rec(&find_root(map), &map, &mut weights);
    weights
}

fn solve_second_part(content: String) -> u32 {
    let map = parse(content);
    println!("{:?}", &real_weights(&map));
    0
}

#[cfg(test)]
mod tests {

    use super::{solve_first_part, solve_second_part};

    const INPUT : &str = "pbga (66)
                          xhth (57)
                          ebii (61)
                          havc (66)
                          ktlj (57)
                          fwft (72) -> ktlj, cntj, xhth
                          qoyq (66)
                          padx (45) -> pbga, havc, qoyq
                          tknk (41) -> ugml, padx, fwft
                          jptl (61)
                          ugml (68) -> gyxo, ebii, jptl
                          gyxo (61)
                          cntj (57)";

    #[test]
    fn seventh_problem_first_part() {
        assert_eq!(solve_first_part(String::from(INPUT)), "tknk");
    }

    #[test]
    fn seventh_problem_second_part() {
        assert_eq!(solve_second_part(String::from(INPUT)), 60);
    }

}
