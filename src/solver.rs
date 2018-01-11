use genetic;

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
    pub price: usize,
    pub configuration: Vec<bool>,
}

fn count_satisfied(x: &Vec<bool>, formula: &Vec<Vec<i16>>) -> usize {
    formula.clone().iter().fold(0, |acc, ref cl| {
        for v in cl.iter() {
            let idx = i16::abs(*v) as usize - 1;
            if (*v < 0 && x[idx]) || (*v > 0 && !x[idx]) { return acc; }
        }
        acc + 1
    })
}

pub fn sum_valid_weights(x: &Vec<bool>, weights: &Vec<u16>) -> usize {
    x.iter().enumerate().fold(0, |sum, (i, val)| {
        if *val { sum + weights[i] } else { sum }
    }) as usize
}

pub fn solve(problem: &Problem) -> Solution {
    let clausules = problem.clausules as f32;
    let k = 0.95;
    
    // Define a fitness function
    let fitness_fn = |x: &Vec<bool>| {
        let satisfied = count_satisfied(x, &problem.formula) as f32;
        let weight_sum = sum_valid_weights(x, &problem.weights) as f32;

        let fitness = 2048.0 * (k * f32::exp(satisfied / clausules) + (1.0 - k) * f32::exp(weight_sum / problem.maximum as f32));
        fitness as usize
    };
    
    // Define a dummy constraint function
    let constraint_fn = |_: &Vec<bool>| { true };

    // Configure the evolution
    let configuration = genetic::EvolutionConfiguration {
        pop_size: 64,
        chrom_size: problem.variables as usize,
        generations: 128,
        xover_probability: 0.5,
        mutation_probability: 0.5,
        tments: 4,
        tment_size: 4,
        elitism: 1,
    };
    
    // Simulate
    let best = genetic::simulate(configuration, &fitness_fn, &constraint_fn);
    let best_price = fitness_fn(&best);
    
    Solution {
        price: best_price,
        configuration: best,
    }
}