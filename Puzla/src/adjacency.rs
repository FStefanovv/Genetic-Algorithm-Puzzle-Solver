use std::collections::HashMap;
use std::vec::Vec;

use image::DynamicImage;

use crate::dissimilarity::calculate_dissimilarity;
use rayon::prelude::*;

pub struct AdjacencyData {
    pieces: HashMap<String, DynamicImage>,
    compatibilities: HashMap<String, HashMap<char, Vec<(f64, String)>>>,
    best_buddies: Vec<(String, String, char)>
}

impl AdjacencyData {
    pub fn new(pieces: &HashMap<String, DynamicImage>) -> Self {
        let mut compatibilities = HashMap::new();
        for (key, _) in pieces {
            compatibilities.insert(key.clone(), HashMap::new());
        }
        let best_buddies = Vec::new();     

        let mut instance = Self { pieces: pieces.clone(), compatibilities, best_buddies };

        instance.calculate_compatibilities();
        instance.generate_best_buddies();

        instance
    }

    fn calculate_compatibilities(&mut self) {
        for (key1, piece1) in &self.pieces {
            for &relation in &['L', 'R', 'U', 'D'] {
                let mut dissimilarities: Vec<(f64, String)> = Vec::new();
    
                for (key2, piece2) in &self.pieces {
                    if key1 != key2 {
                        let dissimilarity = calculate_dissimilarity(&piece1, &piece2, relation);
                        dissimilarities.push((dissimilarity, key2.clone()));
                    }
                }
                
                dissimilarities.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    
                let top_pieces: Vec<(f64, String)> = dissimilarities.iter().take(100).cloned().collect();
    
                self.compatibilities
                    .entry(key1.clone())
                    .or_insert_with(HashMap::new)
                    .insert(relation, top_pieces);
            }
        }
    }

    fn generate_best_buddies(&mut self) {
        let compatibilities_clone = self.compatibilities.clone(); // Clone compatibilities before entering parallel section

        let best_buddies: Vec<_> = compatibilities_clone.par_iter().flat_map(|(key1, compat_map)| {
            compat_map.par_iter().filter_map(|(&relation1, dissimilarities)| {
                if let Some((_dissimilarity, key2)) = dissimilarities.first().cloned() {
                    let relation2 = match relation1 {
                        'L' => 'R',
                        'R' => 'L',
                        'U' => 'D',
                        'D' => 'U',
                        _ => return None,
                    };
    
                    if let Some(compat_key2) = compatibilities_clone.get(&key2).and_then(|map| map.get(&relation2)).map(|v| v.first().map(|(_, k)| k.clone())) {
                        if Some(key1.clone()) == compat_key2 {
                            Some((key1.clone(), key2.clone(), relation1))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
        }).collect();
    
        self.best_buddies.extend(best_buddies);
    }

    

    pub fn get_best_buddy_in_relation(&self, piece: &String, relation: char) -> Option<String> {
        self.best_buddies.iter().find_map(|(buddy1, buddy2, rel)| {
            if *rel == relation && buddy1 == piece {
                Some(buddy2)
            } else {
                None
            }
        }).map(|x| x).cloned()
    }

    pub fn get_most_compatible(&self, piece: &str, relation: char) -> &Vec<(f64, String)> {
        let piece_map = self.compatibilities.get(piece).unwrap();
        piece_map.get(&relation).unwrap()
    }
}