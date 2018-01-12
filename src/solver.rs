use genetic;
use std::f32;

#[derive(Debug, Clone)]
pub struct Problem {
    pub variables: u16,
    pub clausules: u16,
    pub formula: Vec<Vec<i16>>,
    pub weights: Vec<u16>,
    pub maximum: u16,
}

#[derive(Debug, Clone)]
pub struct Solution {
    pub id: usize,
    pub price: usize,
    pub configuration: Vec<bool>,
    pub satisfied: usize,
    pub weight: usize,
}

fn count_satisfied(x: &Vec<bool>, formula: &Vec<Vec<i16>>) -> usize {
    let satisfied = formula.clone().iter().fold(0, |acc, ref cl| {
        for v in cl.iter() {
            let idx = i16::abs(*v) as usize - 1;
            if (*v < 0 && x[idx]) || (*v > 0 && !x[idx]) { return acc + 1; }
        }
        acc
    });
    satisfied
}

pub fn sum_valid_weights(x: &Vec<bool>, weights: &Vec<u16>) -> usize {
    x.iter().enumerate().fold(0, |sum, (i, val)| {
        if *val { sum + weights[i] } else { sum }
    }) as usize
}

pub fn solve(problem: &Problem, sol_id: usize) -> Solution {
    let clausules = problem.clausules as f32;
    let k = 0.85;
    
    // Define a fitness function
    let fitness_fn = |x: &Vec<bool>| {
        let satisfied = count_satisfied(x, &problem.formula) as f32;
        let weight_sum = sum_valid_weights(x, &problem.weights) as f32;

        let fitness = 10000.0 * (k * (satisfied / clausules).exp() + (1.0 - k) * (weight_sum / problem.maximum as f32).exp()).ln();
        fitness as usize
    };
    
    // Define a constraint function (used only for validation)
    let constraint_fn = |x: &Vec<bool>| { count_satisfied(x, &problem.formula) == problem.clausules as usize };

    // Configure the evolution
    let configuration = genetic::EvolutionConfiguration {
        pop_size: 32, // Population size
        chrom_size: problem.variables as usize, // Genome size
        generations: 1024, // Number of generations
        xover_probability: 0.9, // Crossover probability
        mutation_probability: 0.75, // Mutation probability
        tments: 6, // Tournaments
        tment_size: 18, // Tournament size
        elitism: 0, // Elitism
        using_constraint: false, // Are constraints active?
        inspect: true, // Inspect evolution?
    };
    
    // Simulate
    let best = genetic::simulate(configuration, &fitness_fn, &constraint_fn);
    let best_price = fitness_fn(&best);
    
    Solution {
        id: sol_id,
        price: best_price,
        satisfied: count_satisfied(&best, &problem.formula),
        weight: sum_valid_weights(&best, &problem.weights),
        configuration: best,
    }
}