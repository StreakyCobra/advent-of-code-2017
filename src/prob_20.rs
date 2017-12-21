use std::fs::File;
use std::io::Read;
use std::process;
use std::collections::HashMap;
use itertools::Itertools;
use nom::digit;

const CYCLES: usize = 1_000;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Vector {
    x: isize,
    y: isize,
    z: isize,
}

impl Vector {
    fn new(x: isize, y: isize, z: isize) -> Vector {
        Vector { x, y, z }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Particle {
    pos: Vector,
    vel: Vector,
    acc: Vector,
}

impl Particle {
    fn distance(&self) -> usize {
        (self.pos.x.abs() + self.pos.y.abs() + self.pos.z.abs()) as usize
    }

    fn next(&mut self) {
        self.vel.x += self.acc.x;
        self.vel.y += self.acc.y;
        self.vel.z += self.acc.z;
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;
        self.pos.z += self.vel.z;
    }
}

named!(number(&str) -> isize, alt!(recognize!(pair!(opt!(tag!("-")), call!(digit))) => {|s: &str| s.parse().unwrap()}));
named!(xyz(&str) -> Vector, map!(separated_list!(tag!(","), number), |l| Vector::new(l[0], l[1], l[2])));
named!(vector(&str) -> Vector, delimited!(char!('<'), xyz, char!('>')));
named!(particle(&str) -> Particle, ws!(do_parse!(
    tag!("p=") >>
    p: vector >>
    tag!(", v=") >>
    v: vector >>
    tag!(", a=") >>
    a: vector >>
    (Particle {
        pos: p,
        vel: v,
        acc: a,
    }))));

/// Solve the twentieth problem.
pub fn solve() {
    // Open the input file of the problem
    let mut file = match File::open("input/20.txt") {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Error: Input file for the problem 20 not found");
            process::exit(1)
        }
    };
    // Read the file in a String variable
    let mut content: String = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => (),
        Err(_) => {
            eprintln!("Error: Input file for the problem 20 can't be read");
            process::exit(1)
        }
    };
    // Compute and print the solutions of the two parts
    println!("20. Solutions to the twentieth problem:");
    println!("\tFirst part: {}", solve_first_part(parse(&content)));
    println!("\tSecond part: {}", solve_second_part(parse(&content)));
}

fn parse(input: &str) -> Vec<Particle> {
    input.lines()
         .map(|line| particle(line).to_result().unwrap())
         .collect()
}

fn solve_first_part(mut particles: Vec<Particle>) -> usize {
    for _ in 0..CYCLES {
        for mut particle in &mut particles {
            particle.next()
        }
    }
    particles.iter()
             .enumerate()
             .min_by_key(|&(_, v)| v.distance())
             .unwrap()
             .0
}

fn eliminate_duplicates(particles: &mut Vec<Particle>) {
    let mut grps: HashMap<Vector, Vec<Particle>> = HashMap::new();
    for particle in particles.iter() {
        if grps.contains_key(&particle.pos) {
            grps.get_mut(&particle.pos).unwrap().push(particle.clone());
        } else {
            grps.insert(particle.pos.clone(), vec![particle.clone()]);
        }
    }
    let wrongs = grps.iter()
                     .filter(|&(_, v)| v.len() > 1)
                     .map(|(_, v)| v)
                     .flatten();
    for wrong in wrongs {
        particles.retain(|v| *v != *wrong);
    }
}

fn solve_second_part(mut particles: Vec<Particle>) -> usize {
    for _ in 0..CYCLES {
        eliminate_duplicates(&mut particles);
        for mut particle in &mut particles {
            particle.next()
        }
    }
    particles.len()
}

#[cfg(test)]
mod tests {

    use super::{parse, solve_first_part, solve_second_part};

    const INPUT: &str = "p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>
                         p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>";

    #[test]
    fn twentieth_problem_first_part() {
        assert_eq!(solve_first_part(parse(INPUT)), 0);
    }

    const INPUT_2: &str = "p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>
                           p=<-4,0,0>, v=<2,0,0>, a=<0,0,0>
                           p=<-2,0,0>, v=<1,0,0>, a=<0,0,0>
                           p=<3,0,0>, v=<-1,0,0>, a=<0,0,0>";

    #[test]
    fn twentieth_problem_second_part() {
        assert_eq!(solve_second_part(parse(INPUT_2)), 1);
    }

}
