use std::fs::File;
use std::io::Read;
use std::process;
use std::collections::HashMap;
use itertools::Itertools;
use ndarray::{Array2, arr2};

type Mat = Array2<usize>;
type Rules = HashMap<Mat, Mat>;

named!(matrix(&str) -> Vec<&str>, separated_list_complete!(char!('/'), is_a!(".#")));
named!(rule(&str) -> (Vec<&str>, Vec<&str>), ws!(separated_pair!(matrix, tag!("=>"), matrix)));

/// Solve the twenty-oneth problem.
pub fn solve() {
    // Open the input file of the problem
    let mut file = match File::open("input/21.txt") {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Error: Input file for the problem 21 not found");
            process::exit(1)
        }
    };
    // Read the file in a String variable
    let mut content: String = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => (),
        Err(_) => {
            eprintln!("Error: Input file for the problem 21 can't be read");
            process::exit(1)
        }
    };
    // Compute and print the solutions of the two parts
    println!("21. Solutions to the twenty-oneth problem:");
    println!("\tFirst part: {}", solve_generic(parse(&content), 5));
    println!("\tSecond part: {}", solve_generic(parse(&content), 18));
}

fn vec_to_mat(vec: &Vec<&str>) -> Mat {
    let size = vec.len();
    let grid: Vec<usize> = vec.iter()
                           .map(|line| line.chars()
                                           .map(|c| match c {
                                               '#' => 1usize,
                                               _ => 0usize,
                                           })
                                           .collect::<Vec<usize>>())
                           .flatten()
                           .collect();
    Mat::from_shape_vec((size, size), grid).unwrap()
}

fn flip(mat: &Mat) -> Mat {
    let size = mat.shape()[0];
    let mut new = mat.clone();
    for i in 0..size {
        for j in 0..size {
            new[[i, size-1-j]] = mat[[i, j]]
        }
    }
    new
}

fn rot90(mat: &Mat) -> Mat {
    let size = mat.shape()[0];
    let mut new = mat.clone();
    for i in 0..size {
        for j in 0..size {
            new[[j, size-1-i]] = mat[[i, j]]
        }
    }
    new
}

fn rot(mat: &Mat, deg: usize ) -> Mat {
    match deg {
        0 => mat.clone(),
        90 => rot90(mat),
        180 => rot90(&rot90(mat)),
        270 => rot90(&rot90(&rot90(mat))),
        _ => panic!("Wrong degree")
    }
}

fn parse(input: &str) -> HashMap<Mat, Mat> {
    let mut rules = HashMap::new();
    let parsed: Vec<(Mat, Mat)> = 
        input.lines()
            .map(|line| rule(line).to_result().unwrap())
            .map(|(a, b)| (vec_to_mat(&a), vec_to_mat(&b)))
            .collect();
    for (f, t) in parsed {
        for deg in [0, 90, 180, 270].iter() {
            rules.insert(rot(&f, *deg), rot(&t, 0));
            rules.insert(rot(&flip(&f), *deg), rot(&t, 0));
        }
    }
    rules
}

fn solve_generic(val: Rules, cycles: usize) -> usize {
    let mut grid = arr2(&[[0, 1, 0], [0, 0, 1], [1, 1, 1]]);
    for c in 0..cycles {
        let size = grid.shape()[0] as usize;
        let mut steps: usize;
        let mut stepsize: usize;
        // println!("<{}:{}>", steps, stepsize);

        if size % 2 == 0 {
            stepsize = 2;
            steps = size / 2;
        } else if size % 3 == 0 {
            stepsize = 3;
            steps = size / 3;
        } else {
            panic!("Wrong grid size")
        }

        let newsize = steps as usize * (stepsize + 1) as usize;
        let mut newgrid = Mat::zeros((newsize, newsize));
        println!("CYCLE: {} | OUTPUT SIZE: {}", c, newsize);

        for x in 0..steps as usize {
            for y in 0..steps as usize {
                let clone = grid.clone();
                let x_from = (x * stepsize) as isize;
                let y_from = (y * stepsize) as isize;
                let x_to = x_from + stepsize as isize;
                let y_to = y_from + stepsize as isize;
                let sub = clone.slice(s![x_from..x_to, y_from..y_to]);
                // println!("{:?}", &sub);
                let new = val.get(&sub.to_owned()).unwrap();
                // println!("{:?}", &new);
                let size = new.shape()[0];
                for i in 0..size {
                    for j in 0..size {
                        newgrid[[x*size + i, y*size + j]] = new[[i, j]];
                    }
                }
                // println!("{:?}\n", &new);
            }
        }

        grid = newgrid;
        // println!("{:?}\n---------------\n\n", &grid);
    }
    grid.scalar_sum()
}

#[cfg(test)]
mod tests {

    use super::{parse, solve_generic};

    const INPUT: &str = "../.# => ##./#../...\n.#./..#/### => #..#/..../..../#..#\n";

    #[test]
    fn twentyoneth_problem() {
        assert_eq!(solve_generic(parse(INPUT), 2), 12);
    }

}
