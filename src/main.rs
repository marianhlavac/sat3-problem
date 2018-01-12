extern crate time;
extern crate rand;

mod solver;
mod dimacs;
mod genetic;


fn main() {
    let problem = dimacs::read("data/lubr/input-45/uf20-02.cnf");    
    let solution = solver::solve(&problem, 0);
    println!("===\nid,price,variables,clausules,satisfied,weight\n{},{},{},{},{},{}", 
        solution.id,
        solution.price,
        problem.variables,
        problem.clausules,
        solution.satisfied,
        solution.weight,
    );
}