//Filip Stefanov, E2 110-2023

use std::{cmp::Ordering, collections:: HashMap};

use image::{self, DynamicImage, GenericImageView};
use rayon::prelude::*;
use std::sync::{Arc, RwLock};

mod utils;
mod dissimilarity;
mod fitness;
mod init_population;
mod adjacency;
mod selection;
mod crossover;


fn main() {
    let path_to_pieces_directory = "../slika 5";

    let path_to_image = "../picture5.jpg";
    let original_image = image::open(&path_to_image).expect("failed to open original image");

    let pieces_paths = utils::get_directory_contents(path_to_pieces_directory).unwrap();
    let mut loaded_pieces: HashMap<String, DynamicImage> = HashMap::new();
    for piece in pieces_paths {
        let current_image = image::open(&piece).expect("couldnt open piece");
        if current_image.dimensions().0 == 1 || current_image.dimensions().1 == 1 {
            continue;
        }
        if let Some(file_name) = piece.rsplit('/').next() {
            loaded_pieces.insert(file_name.to_string(), current_image);
        }
    }

    let (
        avg_width,
        avg_height,
        _smallest_width,
        _smallest_height,
        _largest_width,
        _largest_height,
    ) = utils::calculate_dimensions_stats(&loaded_pieces).unwrap();
     
    let matrix_width = (original_image.dimensions().0 as f64 / avg_width as f64).round() as u32;
    let matrix_height = (original_image.dimensions().1 as f64 / avg_height as f64).round() as u32;

    let loaded_pieces: HashMap<String, DynamicImage> = loaded_pieces
    .iter()
    .map(|(key, piece)| (key.clone(), utils::resize(piece, avg_width, avg_height)))
    .collect();

    let adjacency = adjacency::AdjacencyData::new(&loaded_pieces);
   
    let (dissimilarity_r, dissimilarity_d) = dissimilarity::calculate_dissimilarity_matrices(&loaded_pieces);

    let population_size = 500;

    let init_pop = init_population::generate_initial_population(loaded_pieces.keys().cloned().collect(), matrix_width as usize, matrix_height as usize, population_size);
    
    let num_of_generations = 30;
    
    let mut current_population = init_pop;

    for i in 0..num_of_generations {
        let mut fitness_scores = fitness::evaluate_generation( &current_population, &dissimilarity_r, &dissimilarity_d);
        
        fitness_scores.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        
        let new_population = Arc::new(RwLock::new(Vec::new()));

        let elite: Vec<Vec<Vec<String>>> = fitness_scores.iter().take(4).map(|&(index, _)| current_population[index].clone()).collect();

        {
            let mut new_population_lock = new_population.write().unwrap();
            new_population_lock.extend(elite);
        }

        (0..population_size - 4).into_par_iter().for_each(|_| {
            let (parent1_idx, parent2_idx) = selection::select_parents(&fitness_scores);
        
            let mut child: Option<Vec<Vec<String>>> = None;
            
            while child.is_none() {
                let mut crossover = crossover::Crossover::new(&current_population[parent1_idx], &current_population[parent2_idx], &adjacency);
                child = crossover.generate_child();
            }
            let mut new_population_lock = new_population.write().unwrap();
            new_population_lock.push(child.unwrap());
        });
       
        current_population = new_population.read().unwrap().clone();
        println!("Generation {}/{} finished", i+1, num_of_generations);
    }
    let fitness_scores_final = fitness::evaluate_generation( &current_population, &dissimilarity_r, &dissimilarity_d);
    if let Some(min_tuple) = fitness_scores_final.iter().min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal)) {
        let max_fitness_index = min_tuple.0;

        let fittest = &current_population[max_fitness_index];
        
        let image_matrix = utils::create_image_matrix(fittest, &loaded_pieces);
        let image = utils::create_image_from_matrix(&image_matrix);
        image.save("solved.png").expect("Failed to save image");
    } else {
        println!("Vector is empty");
    }
}
