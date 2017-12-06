#![feature(iterator_step_by)]

extern crate itertools;

mod prob_01;
mod prob_02;
mod prob_03;
mod prob_04;
mod prob_05;
mod prob_06;

/// Run the solver for the solutions of the Advent Of Code 2017
fn main() {
    println!("Solutions to the Advent of Code 2017");
    println!("====================================\n");
    prob_01::solve();
    prob_02::solve();
    prob_03::solve();
    prob_04::solve();
    prob_05::solve();
    prob_06::solve();
}
