mod utils;

use rand::prelude::*;
use rand::seq::SliceRandom;
use std::mem;

/// Represents a permutation genotype.
/// The elements are in the range 0..N-1 where N is the number of alleles.
pub struct Genotype {
    data: Vec<usize>,
}

impl Genotype {
    pub fn random(num_alleles: usize, rng: &mut ThreadRng) -> Self {
        if num_alleles == 0 {
            panic!("number of alleles may not be 0");
        }
    
        let mut data = (0..num_alleles).collect::<Vec<usize>>();
        data.shuffle(rng);

        Self {
            data,
        }
    }

    pub fn data(&self) -> &[usize] {
        &self.data
    }

    pub fn inversion_mutation(self, rng: &mut ThreadRng) -> Self {
        let mut clone = self.data.clone();
        let num_alleles = clone.len();

        let mut pos1 = rng.gen_range(0..num_alleles);
        let mut pos2 = rng.gen_range(0..num_alleles);
        if pos1 > pos2 {
            mem::swap(&mut pos1, &mut pos2);
        }

        let _ = &clone[pos1..pos2].reverse();

        Self {
            data: clone,
        }
    }

    pub fn edge_crossover(parent1: &mut Self, parent2: &mut Self, 
                          rng: &mut ThreadRng) -> Self {
        let parent1 = parent1.data();
        let parent2 = parent2.data();
        if parent1.len() != parent2.len() {
            panic!("crossover requires both genotypes to have equal size");
        }

        let mut edge_table = utils::construct_edge_table(&parent1, &parent2); 
        
        utils::construct_child(&mut edge_table, rng)
    }

    pub fn print(&self) {
        println!("{:?}", self.data);
    }
}
