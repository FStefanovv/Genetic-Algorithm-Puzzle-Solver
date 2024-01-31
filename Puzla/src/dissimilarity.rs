use std::collections::HashMap;
use rayon::prelude::*;

use image::{DynamicImage, GenericImageView};

pub fn calculate_dissimilarity(piece1: &DynamicImage, piece2: &DynamicImage, relation: char) -> f64 {
    let mut dissimilarity: f64 = 0.0;
    
    match relation {
        'R' => {
            let height = piece1.dimensions().1;
            let piece1_width = piece1.dimensions().0;


            for i in 0..height {
                let pixel1 = piece1.get_pixel(piece1_width-1, i);
                let pixel2 = piece2.get_pixel(0, i);

                let diff_r = f64::from(pixel1[0]) - f64::from(pixel2[0]);
                let diff_g = f64::from(pixel1[1]) - f64::from(pixel2[1]);
                let diff_b = f64::from(pixel1[2]) - f64::from(pixel2[2]);

                let current_diff = diff_r * diff_r + diff_g * diff_g + diff_b * diff_b;
                dissimilarity += current_diff;
            }
        },
        'D' => {
            let width = piece1.dimensions().0;
            let piece1_height = piece1.dimensions().1;

            for i in 0..width {
                let pixel1 = piece1.get_pixel(i, piece1_height-1);
                let pixel2 = piece2.get_pixel(i, 0);

                let diff_r = f64::from(pixel1[0]) - f64::from(pixel2[0]);
                let diff_g = f64::from(pixel1[1]) - f64::from(pixel2[1]);
                let diff_b = f64::from(pixel1[2]) - f64::from(pixel2[2]);

                let current_diff = diff_r * diff_r + diff_g * diff_g + diff_b * diff_b;
                dissimilarity += current_diff;
            }
        },
        'L' => {
            let height = piece1.dimensions().1;

            for i in 0..height {
                let pixel1 = piece1.get_pixel(0, i);
                let pixel2 = piece2.get_pixel(piece2.dimensions().0-1, i);

                let diff_r = f64::from(pixel1[0]) - f64::from(pixel2[0]);
                let diff_g = f64::from(pixel1[1]) - f64::from(pixel2[1]);
                let diff_b = f64::from(pixel1[2]) - f64::from(pixel2[2]);

                let current_diff = diff_r * diff_r + diff_g * diff_g + diff_b * diff_b;
                dissimilarity += current_diff;
            }
        },
        'U' => {
            let width = piece1.dimensions().0;

            for i in 0..width {
                let pixel1 = piece1.get_pixel(i, 0);
                let pixel2 = piece2.get_pixel(i, piece2.dimensions().1-1);

                let diff_r = f64::from(pixel1[0]) - f64::from(pixel2[0]);
                let diff_g = f64::from(pixel1[1]) - f64::from(pixel2[1]);
                let diff_b = f64::from(pixel1[2]) - f64::from(pixel2[2]);

                let current_diff = diff_r * diff_r + diff_g * diff_g + diff_b * diff_b;
                dissimilarity += current_diff;
            }
        },
        _ => {
            dissimilarity = -1.0;
        }
    }

    dissimilarity.sqrt()
}

pub fn calculate_dissimilarity_matrices(
    loaded_pieces: &HashMap<String, DynamicImage>,
) -> (HashMap<(String, String), f64>, HashMap<(String, String), f64>) {
    let mut dissimilarity_matrix_r: HashMap<(String, String), f64> = HashMap::new();
    let mut dissimilarity_matrix_d: HashMap<(String, String), f64> = HashMap::new();


    let dissimilarity_results: Vec<((String, String), f64, f64)> = loaded_pieces
    .par_iter()
    .flat_map(|(key1, piece1)| {
        loaded_pieces.par_iter().filter_map(move |(key2, piece2)| {
            if key1 != key2 {
                let dissimilarity_r = calculate_dissimilarity(piece1, piece2, 'R');
                let dissimilarity_d = calculate_dissimilarity(piece1, piece2, 'D');
                Some(((key1.clone(), key2.clone()), dissimilarity_r, dissimilarity_d))
            } else {
                None
            }
        })
    })
    .collect();

    dissimilarity_matrix_r.extend(dissimilarity_results.iter().map(|&(ref k, v, _)| (k.clone(), v)));
    dissimilarity_matrix_d.extend(dissimilarity_results.iter().map(|&(ref k, _, v)| (k.clone(), v)));

    (dissimilarity_matrix_r, dissimilarity_matrix_d)
}