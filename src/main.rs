use std::env;

mod benchmark;
mod read_file;
mod solutions;
use benchmark::benchmarked_main;
use read_file::{read_all, read_to_tuple};

use solutions::day1::{day1_sol1, day1_sol2};
use solutions::day2::{day2_sol1, day2_sol2};

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1];

    const ITERATIONS: usize = 1;
    match day.as_str() {
        "day1" => {
            benchmarked_main(read_all, day1_sol1, "inputs/day1.in", ITERATIONS, false);
            println!("");
            benchmarked_main(read_all, day1_sol2, "inputs/day1.in", ITERATIONS, false);
        }
        "day2" => {
            benchmarked_main(read_to_tuple, day2_sol1, "inputs/day2.in", ITERATIONS, false);
            println!("");
            benchmarked_main(read_to_tuple, day2_sol2, "inputs/day2.in", ITERATIONS, false);
        }
        _ => println!("Wrong argument!"),
    }
}
