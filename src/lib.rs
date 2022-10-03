mod file_parser;
pub mod config;
mod weighted_graph;
mod genotype;


use rand::prelude::*;
use rand::distributions::WeightedIndex;
use rand::distributions::uniform::SampleUniform;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::io::Write;

use weighted_graph::*;
use genotype::*;
use config::*;

/// Attempts to run the traveling salesperson problem, using the file named
/// by the argument to populate the map.
/// Returns an Error if the string does not represent a file, or if the file
/// is not correctly formatted.
pub fn run(config: Config) 
-> Result<(), &'static str> {
    let file_name = config.file_name;
    let pop_size = config.pop_size;
    let num_evals = config.num_evals;
    
    let mut rng = rand::thread_rng();

    let point_vector = file_parser::parse_file(&file_name)?;
    let graph = WeightedGraph::from_points(point_vector);
    
    let num_alleles = graph.num_vertices();

    // INVARIANT: POPULATION MUST ALWAYS BE SORTED BY FITNESS AT ALL TIMES!!!
    let mut pop_and_fitness = initial_population(
        pop_size, num_alleles, &graph, &mut rng
    );

    let selection_dist = (0..pop_size)
        .rev()
        .map(|rank| probability_by_rank(2.0, rank, pop_size));
    let parent_selection_dist = WeightedIndex::new(
            selection_dist.clone().collect::<Vec<f64>>()
        )
        .unwrap();

    let survivor_selection_dist = WeightedIndex::new(
            selection_dist.rev().collect::<Vec<f64>>()
        )
        .unwrap();
    for gen in 1..=num_evals {

        let parent1 = &pop_and_fitness[parent_selection_dist.sample(&mut rng)];
        let parent2 = &pop_and_fitness[parent_selection_dist.sample(&mut rng)];
        let mut child1 = Genotype::edge_crossover(&parent1.0, &parent2.0, &mut rng);
        
        if gen > 5000 {
            child1 = child1.inversion_mutation(&mut rng);
            let child1_fitness = fitness(&graph, child1.data());
            let child1 = (child1, child1_fitness);
            replace_worst(&mut pop_and_fitness, child1);
        } else {
            child1 = child1.swap_mutation(&mut rng);
            let child1_fitness = fitness(&graph, child1.data());
            let child1 = (child1, child1_fitness);
            replace_with_probability(
                &mut pop_and_fitness, child1, &survivor_selection_dist, &mut rng
            );
        }

        if gen % 10 == 0 {
            println!("{} {}", gen, pop_and_fitness[0].1);
        }
    }

    Ok(())
}

fn try_open_file(file_name: &str) -> Result<File, &'static str> {
    match OpenOptions::new().write(true).open("stats.txt") {
        Err(_) => Err("could not open output file"),
        Ok(file) => Ok(file),
    }
}

fn replace_with_probability<X>(pop_and_fitness: &mut Vec<(Genotype, f64)>,
                            child: (Genotype, f64), 
                            selection_dist: &WeightedIndex<X>, 
                            rng: &mut ThreadRng,
                            ) 
where X: SampleUniform + PartialOrd 
{
    pop_and_fitness.remove(selection_dist.sample(rng));
    let mut idx = 0;
    for x in pop_and_fitness.iter() {
        if child.1 > x.1 {
            idx += 1;
        }
    }
    pop_and_fitness.insert(idx, child);
}

fn replace_worst(pop_and_fitness: &mut Vec<(Genotype, f64)>, 
                 child: (Genotype, f64)) {
    let mut idx = 0;
    for x in pop_and_fitness.iter() {
        if child.1 > x.1 {
            idx += 1;
        }
    }
    pop_and_fitness.insert(idx, child);
    pop_and_fitness.pop();
}

fn initial_population(pop_size: usize, num_alleles: usize, 
                      graph: &WeightedGraph, rng: &mut ThreadRng) 
-> Vec<(Genotype, f64)> {
    let population = (0..pop_size)
        .map(|_| Genotype::random(num_alleles, rng))
        .collect::<Vec<Genotype>>();

    let fitness = population
        .iter()
        .map(|genotype| fitness(&graph, &genotype.data()))
        .collect::<Vec<f64>>();

    let mut pop_fitness = population
        .into_iter()
        .zip(fitness.into_iter())
        .collect::<Vec<(Genotype, f64)>>();
    pop_fitness.sort_by(|x, y| x.1.partial_cmp(&y.1).unwrap());

    pop_fitness
}

fn fitness(environ: &WeightedGraph, individual: &[usize]) -> f64 {
    // Edge from the last city back to the start.
    let final_edge = environ.weight_between(
        individual[0], individual[environ.num_vertices()-1]
    );

    let mut current_vertex = individual[0];
    individual
        .into_iter()
        .fold(final_edge, |acc, &x| {
            let dist = environ.weight_between(current_vertex, x);
            current_vertex = x;
            acc + dist
        })
}

fn probability_by_rank(s: f64, rank: usize, pop_size: usize) -> f64 {
    let rank = rank as f64;
    let pop_size = pop_size as f64;
    (2.0 - s) / pop_size + 
        rank * (2.0 * s - 2.0) / (pop_size.powf(2.0) - pop_size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fitness_is_computed_correctly() {
        let genotype = vec![0, 1, 2];
        let point_vec = vec![
            (0.0, 0.0),
            (3.0, 4.0),
            (6.0, 8.0),
        ];
        let graph = WeightedGraph::from_points(point_vec);

        assert_eq!(20.0, fitness(graph, &genotype));
    }
}
