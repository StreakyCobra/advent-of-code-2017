use std::fs::File;
use std::io::Read;
use std::process;

/// Solve the tenth problem.
pub fn solve() {
    // Open the input file of the problem
    let mut file = match File::open("input/10.txt") {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Error: Input file for the problem 10 not found");
            process::exit(1)
        }
    };
    // Read the file in a String variable
    let mut content: String = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => (),
        Err(_) => {
            eprintln!("Error: Input file for the problem 10 can't be read");
            process::exit(1)
        }
    };
    let lengths: Vec<usize> = content.trim().split(",")
                                            .map(|v| v.parse::<usize>().unwrap())
                                            .collect();
    // Compute and print the solutions of the two parts
    println!("10. Solutions to the tenth problem:");
    println!("\tFirst part: {}", solve_first_part(256, &lengths));
    println!("\tSecond part: {}", solve_second_part(256, &content.trim()));
}

fn circular_slice(from: usize, to: usize, array: &Vec<usize>) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    if from <= to {
        result.extend_from_slice(&array[from..to]);
    } else {
        result.extend_from_slice(&array[from..]);
        result.extend_from_slice(&array[..to]);
    }
    result.reverse();
    result
}

fn solve_first_part(size: usize, lengths: &Vec<usize>) -> usize {
    let mut buf: Vec<usize> = (0..size).collect();
    let mut pos: usize = 0;
    let mut old_pos: usize = 0;
    let mut skip: usize = 0;
    for length in lengths {
        for v in circular_slice(pos, (pos + length) % buf.len(), &buf) {
            buf[pos] = v;
            pos = (pos + 1) % buf.len();
        }
        pos = (old_pos + length + skip) % buf.len();
        old_pos = pos;
        skip += 1;
    }
    match &buf[..] {
        &[a, b, ..] => a * b,
        _ => panic!("Not enough elements in the list to calculate solution"),
    }
}

fn solve_second_part(size: usize, value: &str) -> String {
    let mut lengths: Vec<usize> = value.chars().map(|v| format!("{}", v as u8).parse().unwrap()).collect();
    lengths.extend_from_slice(&vec![17, 31, 73, 47, 23]);
    let mut buf: Vec<usize> = (0..size).collect();
    let mut pos: usize = 0;
    let mut old_pos: usize = 0;
    let mut skip: usize = 0;
    for _ in 0..64 {
        for length in &lengths {
            for v in circular_slice(pos, (pos + length) % buf.len(), &buf) {
                buf[pos] = v;
                pos = (pos + 1) % buf.len();
            }
            pos = (old_pos + length + skip) % buf.len();
            old_pos = pos;
            skip += 1;
        }
    }
    buf.chunks(16)
       .map(|block| block.iter().fold(0, |a, b| a ^ b))
       .map(|xor| format!("{:x}", xor))
       .map(|hex| if hex.len() < 2 { format!("0{}", hex) } else { hex })
       .collect()
}

#[cfg(test)]
mod tests {

    use super::{solve_first_part, solve_second_part};

    #[test]
    fn tenth_problem_first_part() {
        assert_eq!(solve_first_part(5, &vec![3, 4, 1, 5]), 12);
    }

    #[test]
    fn tenth_problem_second_part() {
        assert_eq!(solve_second_part(256, ""), "a2582a3a0e66e6e86e3812dcb672a272");
        assert_eq!(solve_second_part(256, "AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
        assert_eq!(solve_second_part(256, "1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
        assert_eq!(solve_second_part(256, "1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
    }

}
