use std::fs;
use std::collections::HashMap;
use image::{self, imageops, DynamicImage, GenericImage, GenericImageView, RgbaImage};
use rand::seq::SliceRandom;

pub fn build_matrix(
    positions: &HashMap<String, (i32, i32)>,
    min_col: i32,
    max_col: i32,
    min_row: i32,
    max_row: i32,
) -> Vec<Vec<String>> {
    let rows = (max_row - min_row + 1) as usize;
    let cols = (max_col - min_col + 1) as usize;

    let mut matrix: Vec<Vec<String>> = vec![vec!["".to_string(); cols]; rows];

    for (key, (row, col)) in positions {
        let adjusted_row = (*row - min_row) as usize;
        let adjusted_col = (*col - min_col) as usize;

        if adjusted_row < rows && adjusted_col < cols {
            matrix[adjusted_row][adjusted_col] = key.clone();
        }
    }

    matrix
}

pub fn create_random_matrix(
    loaded_piece_keys: &Vec<String>,
    matrix_width: usize,
    matrix_height: usize,
) -> Vec<Vec<String>> {
    let mut rng = rand::thread_rng();
    let mut shuffled_keys = loaded_piece_keys.clone();
    shuffled_keys.shuffle(&mut rng);

    let mut matrix: Vec<Vec<String>> = vec![vec![String::new(); matrix_width]; matrix_height];
    let mut loaded_piece_keys_iter = shuffled_keys.iter().cloned().cycle();

    for i in 0..matrix_height {
        for j in 0..matrix_width {
            if let Some(loaded_piece_key) = loaded_piece_keys_iter.next() {
                matrix[i][j] = loaded_piece_key.clone();
            }
        }
    }

    matrix
}


pub fn create_image_matrix(chromosome: &Vec<Vec<String>>, pieces: &HashMap<String, DynamicImage>) -> Vec<Vec<DynamicImage>> {
    let mut image_matrix: Vec<Vec<DynamicImage>> = Vec::new();

    for row in chromosome {
        let mut image_row: Vec<DynamicImage> = Vec::new();
        for key in row {
            if let Some(image) = pieces.get(key) {
                image_row.push(image.clone());
            } else {
                panic!("Key not found in pieces hashmap: {}", key);
            }
        }
        image_matrix.push(image_row);
    }

    image_matrix
}

pub fn create_image_from_matrix(matrix: &Vec<Vec<DynamicImage>>) -> RgbaImage {
    let mut image_width = 0;
    let mut image_height = 0;

    for row in matrix.iter() {
        let row_width: u32 = row.iter().map(|piece| piece.width()).sum();
        let row_height: u32 = row.iter().map(|piece| piece.height()).max().unwrap_or(0);

        image_width = image_width.max(row_width);
        image_height += row_height;
    }

    let mut final_image = RgbaImage::new(image_width, image_height);

    let mut current_y = 0;
    for row in matrix.iter() {
        let mut current_x = 0;
        for piece in row.iter() {
            final_image.copy_from(piece, current_x, current_y).expect("Error copying image");

            current_x += piece.width();
        }
        current_y += row.iter().map(|piece| piece.height()).max().unwrap_or(0);
    }

    final_image
}

pub fn calculate_dimensions_stats(images: &HashMap<String, DynamicImage>) -> Option<(u32, u32, u32, u32, u32, u32)> {
    let total_images = images.len() as u32;
    let (total_width, total_height, smallest_width,  smallest_height,largest_width, largest_height) =
        images.values().fold((0, 0, u32::MAX, u32::MAX, 0, 0), |acc, img| {
            let width = img.width();
            let height = img.height();

            (
                acc.0 + width,
                acc.1 + height,
                acc.2.min(width),
                acc.3.min(height),
                acc.4.max(width),
                acc.5.max(height),
            )
        });

    if total_images > 0 {
        Some((
            total_width / total_images,
            total_height / total_images,
            smallest_width,
            smallest_height,
            largest_width,
            largest_height,
        ))
    } else {
        None
    }
}

pub fn get_directory_contents(folder_path: &str) -> Result<Vec<String>, std::io::Error> {
    if let Ok(entries) = fs::read_dir(folder_path) {
        let mut paths = Vec::new();

        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(path_str) = path.to_str() {
                    paths.push(path_str.to_string());
                }
            }
        }

        Ok(paths)
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to read directory: {}", folder_path),
        ))
    }
}

pub fn resize(piece: &DynamicImage, target_width: u32, target_height: u32) -> DynamicImage {
    image::DynamicImage::ImageRgba8(imageops::resize(piece, target_width, target_height, image::imageops::FilterType::Nearest))
}

pub fn find_position_in_matrix(parent1: &Vec<Vec<String>>, target: &str) -> Option<(usize, usize)> {
    for (row_idx, row) in parent1.iter().enumerate() {
        for (col_idx, value) in row.iter().enumerate() {
            if value == target {
                return Some((row_idx, col_idx));
            }
        }
    }
    None
}

pub fn matrix_size<T>(matrix: &Vec<Vec<T>>) -> (usize, usize) {
    let num_rows = matrix.len();
    let num_columns = matrix.get(0).map_or(0, |row| row.len());
    (num_rows, num_columns)
}