extern crate time;
extern crate rand;

mod solver;
mod dimacs;
mod genetic;

fn count_success(problem: &solver::Problem, solution: &solver::Solution) -> f32 {
    solver::sum_valid_weights(&solution.configuration, &problem.weights) as f32 / problem.maximum as f32
}

fn main() {
    let problem = dimacs::read("data/easier.cnfw");
    println!("{:?}", problem);
    
    let solution = solver::solve(&problem);
    println!("{:?}, success rate {}%", solution, count_success(&problem, &solution) * 100.0);
}