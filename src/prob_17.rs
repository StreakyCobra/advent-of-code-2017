/// Solve the seventeenth problem.
pub fn solve() {
    // Compute and print the solutions of the two parts
    println!("17. Solutions to the seventeenth problem:");
    println!("\tFirst part: {}", solve_first_part(348));
    println!("\tSecond part: {}", solve_second_part(348));
}

fn solve_first_part(step: usize) -> usize {
    let mut buffer: Vec<usize> = vec![0];
    let mut pos: usize = 0;

    for i in 0..2017 {
        pos = (pos + step) % buffer.len() + 1;
        buffer.insert(pos, i + 1)
    }

    buffer[pos+1]
}

fn solve_second_part(step: usize) -> usize {
    let mut buflen = 1;
    let mut pos = 0;
    let mut result = 0;

    for i in 0..50_000_000 {
        pos = (pos + step) % buflen + 1;
        buflen += 1;
        if pos == 1 {
            result = i + 1
        }
    }

    result
}

#[cfg(test)]
mod tests {

    use super::{solve_first_part, solve_second_part};

    #[test]
    fn seventeenth_problem_first_part() {
        assert_eq!(solve_first_part(3), 638);
    }

}
