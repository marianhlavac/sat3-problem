use genetic;
use time::PreciseTime as ptime;

struct Problem {
    id: usize,
    variables: u16,
    clausules: u16,
    formula: Vec<Vec<u16>>,
    weights: Vec<u8>,
}

pub fn solve(problem: Problem) {
    // Configure the evolution
    let pop_size = 128; // TODO: Change this
    
    // Define a fitness function
    let fitness_fn = |x: &Vec<bool>| {
        x.iter().enumerate().fold(0, |sum, (i, val)| {
            if val { sum + problem.weights[i] } else { sum }
        }) as usize
    };
    
    // Define a constraint function
    let constraint_fn = |x: &Vec<bool>| {
        let mut satisfied = true;
        
        // Iterate through each clausule
        'o: for cl in problem.formula {
            // and find any unsatisfied
            for v in cl if (v < 0 && x[v]) || (v > 0 && !x[v]) {
                satisfied = false; break 'o;
            }
        }
        
        satisfied
    };
    
    // Simulate
    let best = genetic::simulate(pop_size, problem.variables, &fitness_fn, &constraint_fn);
}