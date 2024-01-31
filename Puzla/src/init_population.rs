use crate::utils::create_random_matrix;


pub fn generate_initial_population(pieces: Vec<String>, width: usize, height: usize, size: u32) -> Vec<Vec<Vec<String>>> {
    let mut population: Vec<Vec<Vec<String>>> = Vec::new();

    for _ in 0..size {
        let current = create_random_matrix(&pieces, width, height);
        population.push(current);
    }
    
    population
}