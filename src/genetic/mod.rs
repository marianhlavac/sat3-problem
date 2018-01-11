mod crossover;
mod mutators;
mod selectors;
mod utils;

use self::utils::{odds_are, random_individual, inspect, sort_population};

pub struct EvolutionConfiguration {
    pub pop_size: usize,
    pub chrom_size: usize,
    pub generations: usize,
    pub xover_probability: f32,
    pub mutation_probability: f32,
    pub tments: usize,
    pub tment_size: usize,
    pub elitism: usize,
}

/// Simulates an evolution with genetic algorithm.
pub fn simulate<F, C>(cfg: EvolutionConfiguration, fitness_fn: &F, constr_fn: &C) -> Vec<bool>
where F: Fn(&Vec<bool>) -> usize, C: Fn(&Vec<bool>) -> bool {
    let mut population = utils::create_population(cfg.pop_size, cfg.chrom_size);
    
    // Run for a number of generations
    for i in 0..cfg.generations {
        // Selection
        let mut sorted_population = sort_population(&population, fitness_fn);
        inspect(i, &sorted_population, &fitness_fn);
        
        // Selection
        let mut new_population = selectors::tournament(&population, cfg.tments, cfg.tment_size, fitness_fn);
        
        // Elitism
        sorted_population.truncate(cfg.elitism);
        new_population.append(&mut sorted_population);
        
        // Fill the rest of population with offsprings
        while new_population.len() != cfg.pop_size {
            let mut child: Vec<bool>;
            
            // Mate or select a random individual
            if odds_are(cfg.xover_probability) {
                // Select two random individuals
                let in1 = random_individual(&new_population).clone();
                let in2 = random_individual(&new_population).clone();
                
                // Crossover
                child = crossover::single_point(in1, in2);
            } else {
                child = random_individual(&population);
            }
            
            // Mutation
            mutators::random_inverse(&mut child, cfg.mutation_probability);
            
            // Append to the rest of the population, if valid
            if constr_fn(&child) {
                new_population.push(child);
            }
        }
        
        // Replace the current population with new population
        population = new_population;
    }
    
    // Return the best result
    let sorted_population = sort_population(&population, fitness_fn);
    sorted_population.first().unwrap().clone()
}