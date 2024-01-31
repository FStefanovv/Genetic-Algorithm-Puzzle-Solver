use std::collections::HashMap;

use rayon::prelude::*;

pub fn evaluate_generation(generation: &Vec<Vec<Vec<String>>>, dissimilarities_r: &HashMap<(String, String), f64>, dissimilarities_d: &HashMap<(String, String), f64>) -> Vec<(usize, f64)>{
    
    let fitness_scores: Vec<(usize, f64)> = generation
        .par_iter() 
        .enumerate()
        .map(|(i, chromosome)| {
            let current_fitness = calculate_fitness(chromosome, &dissimilarities_r, &dissimilarities_d);
            (i, current_fitness)
        })
        .collect();

    fitness_scores
}

pub fn calculate_fitness(chromosome: &Vec<Vec<String>>, dissimilarities_r: &HashMap<(String, String), f64>, dissimilarities_d: &HashMap<(String, String), f64>)  -> f64 {
    let rows = chromosome.len();
    let cols = chromosome.first().map_or(0, |row| row.len());
    let mut horizontal_dissimilarity = 0.0;
    let mut vertical_dissimilarity = 0.0;

    rayon::scope(|s| {
        s.spawn(|_| {
            for i in 0..rows {
                for j in 0..cols - 1 {
                    let key1 = &chromosome[i][j];
                    let key2 = &chromosome[i][j + 1];
                    if let Some(&dissimilarity) =
                        dissimilarities_r.get(&(key1.clone(), key2.clone()))
                    {
                        horizontal_dissimilarity += dissimilarity;
                    } else {
                        panic!(
                            "Dissimilarity not found in dissimilarities_r for keys {:?} and {:?}",
                            key1, key2
                        );
                    }
                }
            }
        });

        s.spawn(|_| {
            for i in 0..rows - 1 {
                for j in 0..cols {
                    let key1 = &chromosome[i][j];
                    let key2 = &chromosome[i + 1][j];
                    if let Some(&dissimilarity) =
                        dissimilarities_d.get(&(key1.clone(), key2.clone()))
                    {
                        vertical_dissimilarity += dissimilarity;
                    } else {
                        panic!(
                            "Dissimilarity not found in dissimilarities_d for keys {:?} and {:?}",
                            key1, key2
                        );
                    }
                }
            }
        });
    });

    horizontal_dissimilarity + vertical_dissimilarity
}

