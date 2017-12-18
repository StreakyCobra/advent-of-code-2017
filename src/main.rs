#![feature(iterator_step_by)]
#![feature(entry_or_default)]
#![feature(advanced_slice_patterns, slice_patterns)]

extern crate itertools;

use std::env;

mod prob_01;
mod prob_02;
mod prob_03;
mod prob_04;
mod prob_05;
mod prob_06;
mod prob_07;
mod prob_08;
mod prob_09;
mod prob_10;
mod prob_11;
mod prob_12;
mod prob_13;
mod prob_14;
mod prob_15;
mod prob_16;
mod prob_17;

/// Run the solver for the solutions of the Advent Of Code 2017
fn main() {
    let mut problem: Option<u32> = None;
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        problem = args[1].parse::<u32>().ok();
    }
    println!("Solutions to the Advent of Code 2017");
    println!("====================================\n");
    match problem {
        Some(1) => prob_01::solve(),
        Some(2) => prob_02::solve(),
        Some(3) => prob_03::solve(),
        Some(4) => prob_04::solve(),
        Some(5) => prob_05::solve(),
        Some(6) => prob_06::solve(),
        Some(7) => prob_07::solve(),
        Some(8) => prob_08::solve(),
        Some(9) => prob_09::solve(),
        Some(10) => prob_10::solve(),
        Some(11) => prob_11::solve(),
        Some(12) => prob_12::solve(),
        Some(13) => prob_13::solve(),
        Some(14) => prob_14::solve(),
        Some(15) => prob_15::solve(),
        Some(16) => prob_16::solve(),
        Some(17) => prob_17::solve(),
        Some(_) => panic!("Solution not implemented… yet?"),
        None => {
            prob_01::solve();
            prob_02::solve();
            prob_03::solve();
            prob_04::solve();
            prob_05::solve();
            prob_06::solve();
            prob_07::solve();
            prob_08::solve();
            prob_09::solve();
            prob_10::solve();
            prob_11::solve();
            prob_12::solve();
            prob_13::solve();
            prob_14::solve();
            prob_15::solve();
            prob_16::solve();
            prob_17::solve();
        }
    }
}
