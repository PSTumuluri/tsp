mod utils;

use rand::prelude::*;
use rand::seq::SliceRandom;
use std::mem;
use std::collections::HashSet;

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
    pub fn edge_crossover(parent1: &Self, parent2: &Self, 
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

        eprintln!("Parent1 = {:?}", parent1);
        eprintln!("Parent2 = {:?}", parent2);

        // Random vertices that have not yet been added. Used if following 
        // edges leads to a dead-end.
        let mut not_removed: HashSet<usize> = 
            HashSet::from_iter((0..num_alleles));

        let mut allele = vertex.unwrap(); // literally cannot be None
        child.push(allele);
        utils::remove_edge(&mut edge_table, allele);
        not_removed.remove(&allele);
        eprintln!("Removing {} by random chance", child[0]);

        // Used for combing through the genotype for edges if the current path
        // gets stuck.
        let mut allele_try_idx = 0;

        while child.len() != num_alleles { 
            vertex = utils::try_select_adjacent(&edge_table, allele, rng);
            while let None = vertex {
                if allele_try_idx < child.len() {
                    eprintln!("Checking list for {}", child[allele_try_idx]);
                    vertex = utils::try_select_adjacent(
                        &edge_table, child[allele_try_idx], rng
                    );
                    allele_try_idx += 1; 
                } else {
                    // Got stuck; all child alleles lead to dead-ends
                    allele = utils::select_random(&not_removed, rng);
                    vertex = Some(allele)
                }
            }
            allele = vertex.unwrap();
            eprintln!("Removing {} (edge list: {:?}", allele, edge_table[allele]);
            child.push(allele);
            utils::remove_edge(&mut edge_table, allele);
            not_removed.remove(&allele);
        }

        Genotype { data: child }   
    }

    pub fn print(&self) {
        println!("{:?}", self.data);
    }
}
