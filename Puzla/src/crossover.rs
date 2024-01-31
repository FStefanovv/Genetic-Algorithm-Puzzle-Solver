use std::collections::{HashMap, BinaryHeap};
use std::cmp::{Ordering, Reverse};

use rand::Rng;

use crate::adjacency::AdjacencyData;
use crate::utils::{self, build_matrix};

use ordered_float::OrderedFloat;


#[derive(Debug, PartialEq, Eq)]
struct Candidate(String, (i32, i32), (String, String), OrderedFloat<f64>);

impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.3.cmp(&other.3)
    }
}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const MUTUALLY_AGREED_PRIORITY: OrderedFloat<f64> = OrderedFloat(-2.0);
const BUDDY_PRIORITY: OrderedFloat<f64> = OrderedFloat(-1.0);

pub struct Crossover<'a>  {
    rows: usize,
    columns: usize,
    occupied_positions: Vec<(i32, i32)>,
    kernel:  HashMap<String, (i32, i32)>,
    candidates: BinaryHeap<Reverse<Candidate>>,
    parent1: &'a Vec<Vec<String>>,
    parent2: &'a Vec<Vec<String>>,
    adjacency: &'a AdjacencyData,
    max_row: i32,
    max_col: i32, 
    min_row: i32,
    min_col: i32
}

impl<'a> Crossover<'a> {

    pub fn new(parent1: &'a Vec<Vec<String>>, parent2: &'a Vec<Vec<String>>, adjacency: &'a AdjacencyData) -> Self {
        let rows = parent1.len();
        let columns = parent1.get(0).map_or(0, |row| row.len());
    
        let occupied_positions: Vec<(i32, i32)> = Vec::new();
    
        let candidates = BinaryHeap::new();

        let kernel: HashMap<String, (i32, i32)> = HashMap::new();

        let max_row = 0;
        let min_row = 0;
        let max_col = 0;
        let min_col = 0;
 

        Self {
            parent1, parent2, rows, columns, occupied_positions, 
            kernel, adjacency, min_col, max_col, min_row, max_row,
            candidates
        }
    }

    pub fn generate_child(&mut self) -> Option<Vec<Vec<String>>> {
        let root_piece = self.generate_root_piece();

        self.add_to_kernel(root_piece, (0, 0));

        while self.candidates.len() != 0 {
            let Reverse(candidate) = self.candidates.pop().unwrap();

            if self.occupied_positions.contains(&candidate.1) {
                continue;
            }
            
            if !self.piece_is_available(&candidate.0) {
                self.add_candidate(&candidate.2.0, candidate.2.1, candidate.1);
                continue;
            }

            self.add_to_kernel(candidate.0.clone(), candidate.1);
        }
        
        
        if self.kernel.len() != self.rows * self.columns {
            return None;
        }
         
        Some(build_matrix(&self.kernel, self.min_col, self.max_col, self.min_row, self.max_row))
    }


    fn add_to_kernel(&mut self, piece: String, position: (i32, i32)) {
        self.kernel.insert(piece.clone(), position);
        self.occupied_positions.push(position);
        self.add_candidates(piece, position);
    }

    fn add_candidates(&mut self, piece: String, position: (i32, i32)) {
        let new_boundaries = self.get_newly_available_boundaries(position);
        for (relation, boundary_position) in new_boundaries {
            self.add_candidate(&piece, relation, boundary_position);
        } 
    }

    fn add_candidate(&mut self, piece: &String, relation: String, position: (i32, i32)) {
        let mutually_agreed = self.get_mutually_agreed_piece(&piece, &relation);
        if let Some(agreed) = mutually_agreed {
            if self.piece_is_available(&agreed) {
                let candidate = Candidate(agreed, position, (piece.clone(), relation.clone()), MUTUALLY_AGREED_PRIORITY);
                self.candidates.push(Reverse(candidate));
                return;
            }
        }

        let best_buddy = self.get_best_buddy(piece, &relation);
        if let Some(buddy) = best_buddy {
            if self.piece_is_available(&buddy) {
                let candidate = Candidate(buddy, position, (piece.clone(), relation.clone()), BUDDY_PRIORITY);
                self.candidates.push(Reverse(candidate));
                return;
            }
        }

        let most_compatible = self.adjacency.get_most_compatible(&piece, relation.chars().next().unwrap());
        for (dissimilarity, ref compatible_piece) in most_compatible.iter() {
            if self.piece_is_available(compatible_piece) {
                let candidate = Candidate(compatible_piece.clone(), position, (piece.clone(), relation.clone()), OrderedFloat(*dissimilarity));
                self.candidates.push(Reverse(candidate));
                return;
            }
        }
    }

    fn get_best_buddy(&self, piece: &String, relation: &String) -> Option<String> {
        let best_buddy = self.adjacency.get_best_buddy_in_relation(&piece, relation.chars().next().unwrap());
        
        if best_buddy == None {
            return None;
        }

        let best_buddy = best_buddy.unwrap();
        let parent1_contains_bb = self.contains_best_buddies(self.parent1, &piece, &best_buddy, &relation);
        if parent1_contains_bb {
            return Some(best_buddy);
        }

        let parent2_contains_bb = self.contains_best_buddies(self.parent2, &piece, &best_buddy, &relation);
        if parent2_contains_bb {
            return Some(best_buddy);
        }
        
        None
    }

    fn contains_best_buddies(&self, image: &Vec<Vec<String>>, buddy1: &String, buddy2: &String, relation: &str) -> bool {
        let (row, col) = utils::find_position_in_matrix(image, buddy1).unwrap();
        let image_dims = utils::matrix_size(image);

        let check_position = match relation {
            "U" => {
                if row > 0 {
                    Some((row - 1, col))
                } else {
                    None
                }
            }
            "L" => {
                if col > 0 {
                    Some((row, col - 1))
                } else {
                    None
                }
            }
            "R" => {
                if col < image_dims.1 - 1 {
                    Some((row, col + 1))
                } else {
                    None
                }
            }
            "D" => {
                if row < image_dims.0 - 1 {
                    Some((row + 1, col))
                } else {
                    None
                }
            }
            _ => None,
        };
        match check_position {
            Some((check_row, check_col)) => {
                image.get(check_row).map_or(false, |row| row.get(check_col) == Some(buddy2))
            }
            None => false,
        }

    }
 
    fn get_mutually_agreed_piece(&mut self, piece: &String, relation: &String) -> Option<String> {
        let image_dims = utils::matrix_size(&self.parent1);
        let (row, col) = utils::find_position_in_matrix(self.parent1, &piece).unwrap();
        let check_position = match relation.as_str() {
            "U" => {
                if row > 0 {
                    Some((row - 1, col))
                } else {
                    None
                }
            }
            "L" => {
                if col > 0 {
                    Some((row, col - 1))
                } else {
                    None
                }
            }
            "R" => {
                if col < image_dims.1 - 1 {
                    Some((row, col + 1))
                } else {
                    None
                }
            }
            "D" => {
                if row < image_dims.0 - 1 {
                    Some((row + 1, col))
                } else {
                    None
                }
            }
            _ => None,
        };

        if check_position == None {
            return None;
        }

        let check_position = check_position.unwrap();

        let piece_in_parent1 =  self.parent1.get(check_position.0)?.get(check_position.1).unwrap();
        
        let (row, col) = utils::find_position_in_matrix(self.parent2, &piece).unwrap();

        let check_position = match relation.as_str() {
            "U" => {
                if row > 0 {
                    Some((row - 1, col))
                } else {
                    None
                }
            }
            "L" => {
                if col > 0 {
                    Some((row, col - 1))
                } else {
                    None
                }
            }
            "R" => {
                if col < image_dims.1 - 1 {
                    Some((row, col + 1))
                } else {
                    None
                }
            }
            "D" => {
                if row < image_dims.0 - 1 {
                    Some((row + 1, col))
                } else {
                    None
                }
            }
            _ => None,
        };

        if check_position == None {
            return None;
        }
        let check_position = check_position.unwrap();

        let piece_in_parent2 =  self.parent2.get(check_position.0)?.get(check_position.1).unwrap();

        if piece_in_parent1 == piece_in_parent2 {
            return Some(piece_in_parent1.clone());
        }

        None
    } 

    fn get_newly_available_boundaries(&mut self, position: (i32, i32)) -> Vec<(String, (i32, i32))> {
        let (row, column) = position;
        let mut boundaries = Vec::new();

        if !self.kernel_is_full() {
            let positions: HashMap<&str, (i32, i32)> = [
                ("U", (row - 1, column)),
                ("R", (row, column + 1)),
                ("D", (row + 1, column)),
                ("L", (row, column - 1)),
            ]
            .iter()
            .cloned()
            .collect();
                        
            for (orientation, position) in positions.iter() {

                if !self.occupied_positions.contains(position) && self.is_in_range(*position) {
                    self.update_kernel_boundaries(*position,);
                    boundaries.push((orientation.to_string(), *position));
                }
            }
        }
        boundaries
    }

    fn update_kernel_boundaries(&mut self, row_and_column: (i32, i32)) {
        let (row, column) = row_and_column;
        self.min_row = self.min_row.min(row);
        self.max_row = self.max_row.max(row);
        self.min_col = self.min_col.min(column);
        self.max_col = self.max_col.max(column);
    }

    fn is_in_range(&self, row_and_column: (i32, i32)) -> bool {
        let (row, column) = row_and_column;
        self.is_row_in_range(row) && self.is_column_in_range(column)
    }

    fn is_row_in_range(&self, row: i32) -> bool {
        let current_rows = (self.min_row.min(row)).abs() + (self.max_row.max(row)).abs();
        current_rows < self.rows as i32
    }

    fn is_column_in_range(&self, column: i32) -> bool {
        let current_columns = (self.min_col.min(column)).abs() + (self.max_col.max(column)).abs();
        current_columns < self.columns as i32
    }

    fn kernel_is_full(&self) -> bool {
        self.kernel.len() == self.rows * self.columns
    }

    fn piece_is_available(&self, piece: &String) -> bool {
        !self.kernel.contains_key(piece)
    }

    fn generate_root_piece(&self) -> String {
        let mut rng = rand::thread_rng();
    
        let random_row = rng.gen_range(0..self.rows);
        let random_col = rng.gen_range(0..self.columns);
    
        let root_piece = self.parent1.get(random_row).and_then(|row| row.get(random_col));
    
        root_piece.cloned().unwrap_or_default()
    }
}
