mod crossover;
mod mutators;
mod selectors;
mod utils;

use self::utils::{odds_are, random_individual, inspect, sort_population};
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

pub struct EvolutionConfiguration {
    pub pop_size: usize,
    pub chrom_size: usize,
    pub generations: usize,
    pub xover_probability: f32,
    pub mutation_probability: f32,
    pub tments: usize,
    pub tment_size: usize,
    pub elitism: usize,
    pub using_constraint: bool,
    pub inspect: bool,
}

fn print_bitvec(pool: &Vec<Vec<bool>>, desc: &str) {
    print!("  {}:\n      -", desc);
    for n in pool {
        print!("[");
        for m in n {
            print!("{}", if *m { "1" } else { "0" });
        }
        print!("], ");
    }
    print!("\n");
}

/// Simulates an evolution with genetic algorithm.
pub fn simulate<F, C>(cfg: EvolutionConfiguration, fitness_fn: &F, constr_fn: &C) -> Vec<bool>
where F: Fn(&Vec<bool>) -> usize, C: Fn(&Vec<bool>) -> bool {
    let mut population = utils::create_population(cfg.pop_size, cfg.chrom_size);
    
    // Create inspection file if needed
    // let mut insp_file = match File::create(Path::new("data/out/lorem_ipsum.txt")) {
    //     Err(why) => panic!("couldn't create inspection file: {}", why.description()),
    //     Ok(file) => file,
    // };
    
    // Run for a number of generations
    for i in 0..cfg.generations {
        // Inspection
        let mut sorted_population = sort_population(&population, fitness_fn);
        if cfg.inspect {
            inspect(i, &sorted_population, &fitness_fn);
        }
        //println!("=== GEN {}", i);
        //print_bitvec(&sorted_population, "before");
        
        // Selection
        let mut new_population = vec![];
        //print_bitvec(&new_population, "newpop");
        
        // Elitism
        sorted_population.truncate(cfg.elitism);
        new_population.append(&mut sorted_population);
        
        // Fill the rest of population with offsprings
        while new_population.len() != cfg.pop_size {
            let mut child: Vec<bool>;
            
            // Mate or select a random individual
            if odds_are(cfg.xover_probability) {
                // Select two random individuals
                let in1 = selectors::tournament(&population, 1, cfg.tment_size, fitness_fn);
                let in2 = selectors::tournament(&population, 1, cfg.tment_size, fitness_fn);
                
                // Crossover
                child = crossover::uniform(in1, in2, 0.5);
            } else {
                child = random_individual(&population);
            }
            
            // Mutation
            mutators::random_inverse(&mut child, cfg.mutation_probability);
            
            // Append to the rest of the population, if valid
            if !cfg.using_constraint || constr_fn(&child) {
                new_population.push(child);
                //print_bitvec(&new_population, "aftercross");
            }
        }
        
        // Replace the current population with new population
        population = new_population;
    }
    
    // Sort the resulting population
    let sorted_population = sort_population(&population, fitness_fn);

    // Return the first valid best result
    for individual in &sorted_population {
        if constr_fn(&individual) {
            return individual.clone()
        }
    }
    sorted_population.first().unwrap().clone()
}