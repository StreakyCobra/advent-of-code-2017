use std::collections::HashMap;

/// Solve the third problem.
pub fn solve() {
    // Compute and print the solutions of the two parts
    println!("3. Solutions to the third problem:");
    println!("\tFirst part: {}", solve_first_part(368078));
    println!("\tSecond part: {}", solve_second_part(368078));
}

fn find_pos(value : u32) -> (i32, i32) {
    let mut current : u32 = 1;
    let mut i : i32 = 1;
    loop {
        if current == value { return (0, 0) }
        for v in (1-i)..i {
            current += 1;
            if current == value { return (i, v) };
        }
        for v in (0-i)..(i + 1) {
            current += 1;
            if current == value { return (0-v, i) };
        }
        for v in (1-i)..i {
            current += 1;
            if current == value { return (0-i, 0-v) };
        }
        for v in (0-i)..(i + 1) {
            current += 1;
            if current == value { return (v, 0-i) };
        }
        
        i += 1;
    }
}

/// Solve the first part of the third problem.
fn solve_first_part(value : u32) -> i32 {
    let (x, y) = find_pos(value);
    x.abs() + y.abs()
}

fn sum_around(map : &HashMap<(i32, i32), i32>, x : i32, y : i32) -> i32 {
    let mut total : i32 = 0;
    for i in (0-1)..2 {
        for j in (0-1)..2 {
            total += map.get(&(x + i, y + j)).unwrap_or(&0);
        }
    }
    total
}

/// Solve the second part of the third problem.
fn solve_second_part(value : i32) -> i32 {
    let mut result = HashMap::new();
    let mut i : i32 = 1;
    result.insert((0,0), 1);
    loop {
        for v in (1-i)..i {
            let sum = sum_around(&result, i, v);
            if sum > value { return sum };
            result.insert((i, v), sum);
        }
        for v in (0-i)..(i + 1) {
            let sum = sum_around(&result, 0-v, i);
            if sum > value { return sum };
            result.insert((0-v, i), sum);
        }
        for v in (1-i)..i {
            let sum = sum_around(&result, 0-i, 0-v);
            if sum > value { return sum };
            result.insert((0-i, 0-v), sum);
        }
        for v in (0-i)..(i + 1) {
            let sum = sum_around(&result, v, 0-i);
            if sum > value { return sum };
            result.insert((v, 0-i), sum);
        }
        
        i += 1;
    }
}

#[cfg(test)]
mod tests {

    use super::{solve_first_part, solve_second_part};

    #[test]
    fn third_problem_first_part() {
        assert_eq!(solve_first_part(1), 0);
        assert_eq!(solve_first_part(12), 3);
        assert_eq!(solve_first_part(23), 2);
        assert_eq!(solve_first_part(1024), 31);
    }

    #[test]
    fn third_problem_second_part() {
        assert_eq!(solve_second_part(20), 23);
        assert_eq!(solve_second_part(100), 122);
        assert_eq!(solve_second_part(200), 304);
        assert_eq!(solve_second_part(500), 747);
    }

}
