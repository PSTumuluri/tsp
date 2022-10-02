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

    pub fn num_alleles(&self) -> usize {
        self.data.len()
    }

    /// Returns the allele at the specified position in the genotype.
    /// Panics if the index is out of bounds.
    pub fn allele(&self, pos: usize) -> usize {
        self.data[pos]
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

    /// Constructs a child genotype according to the edge crossover algorithm.
    /// While the child has not been fully constructed, it attempts to add 
    /// adjacent edges first, favoring those common to both parents, then 
    /// accepting those found in one parent or the other, and finally 
    /// resorting to random edges in case the above two cases fail.
    pub fn edge_crossover(parent1: &mut Self, parent2: &mut Self, 
                          rng: &mut ThreadRng) -> Self {
        let parent1 = parent1.data();
        let parent2 = parent2.data();
        if parent1.len() != parent2.len() {
            panic!("crossover requires both genotypes to have equal size");
        }
        let num_alleles = parent1.len();

        let mut edge_table = utils::construct_edge_table(&parent1, &parent2); 
        let mut child = Vec::with_capacity(num_alleles);

        let mut vertex = Some(rng.gen_range(0..num_alleles));

        let allele = vertex.unwrap(); // literally cannot be None
        child.push(allele);
        utils::remove_edge(&mut edge_table, allele);

        // Used for combing through the genotype for edges if we get stuck.
        let mut try_allele = 0;

        while child.len() != num_alleles {
            match vertex {
                Some(v) => {
                    vertex = utils::try_select_adjacent(&edge_table, v, rng);
                },
                None => {
                    if try_allele < child.len() {
                        vertex = utils::try_select_adjacent(
                            &edge_table, child[try_allele], rng
                        );
                        try_allele += 1;
                    }
                }
            }
            let allele = vertex.expect(
                "should be able to find vertices in edge table"
            );
            child.push(allele);
            utils::remove_edge(&mut edge_table, allele);
        }

        Genotype { data: child }   
    }

    pub fn print(&self) {
        println!("{:?}", self.data);
    }
}
