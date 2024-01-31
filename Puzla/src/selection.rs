use std::vec::Vec;

use rand::Rng;

pub fn select_parents(population: &Vec<(usize, f64)>) -> (usize, usize) {
    let parent1 = choose_one(&population).unwrap();
    let parent2 = choose_one(&population).unwrap();

    (parent1, parent2)
}

fn choose_one(population: &Vec<(usize, f64)>) -> Option<usize> {
    let inverted_fitness: Vec<(usize, f64)> = population
        .iter()
        .map(|&(id, fitness)| (id, 1.0 / fitness))
        .collect();

    let total_inverted_fitness: f64 = inverted_fitness.iter().map(|&(_, fitness)| fitness).sum();
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(0.0..total_inverted_fitness);

    let mut cumulative_inverted_fitness = 0.0;

    for &(id, inverted_fitness) in &inverted_fitness {
        cumulative_inverted_fitness += inverted_fitness;
        if cumulative_inverted_fitness >= random_number {
            return Some(id);
        }
    }

    None
}
