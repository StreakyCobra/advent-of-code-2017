extern crate itertools;

mod prob_01;
mod prob_02;

/// Run the solver for the solutions of the Advent Of Code 2017
fn main() {
    println!("Solutions to the Advent of Code 2017");
    println!("====================================\n");
    prob_01::solve();
    prob_02::solve();
}
