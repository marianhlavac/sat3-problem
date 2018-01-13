use genetic::utils;

pub fn tournament<F>(population: &Vec<Vec<bool>>, tournaments: usize, 
    pool_size: usize, fitness_fn: &F) -> Vec<bool>
where F: Fn(&Vec<bool>) -> usize {
    let mut pool = Vec::new();

    // Select random individuals
    for _ in 0..pool_size {
        pool.push(utils::random_individual(population));
    }
    
    // Let them fight! And select the best.
    utils::sort_population(&pool, fitness_fn).first().unwrap().clone()
}