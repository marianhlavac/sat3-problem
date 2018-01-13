extern crate time;
extern crate rand;

use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;

mod solver;
mod dimacs;
mod genetic;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        panic!("Specify the arguments!");
    }

    let filename = &args[1];
    let id = args[2].parse::<usize>().unwrap_or(0);

    let mut resfile = OpenOptions::new().write(true).append(true).open("data/out/results.csv").unwrap();

    let problem = dimacs::read(filename);    
    println!("inst-{}-fitness",id);
    let solution = solver::solve(&problem, 0, true);

    if let Err(e) = writeln!(resfile, "{},{},{},{},{},{},{}", 
        id,
        solution.price,
        problem.variables,
        problem.clausules,
        solution.satisfied,
        solution.weight,
        solution.valid,
    ) {
        println!("{}", e);
    }
}