mod file_parser;
mod weighted_graph;
mod genotype;

use rand::prelude::*;
use rand::distributions::WeightedIndex;

use weighted_graph::*;
use genotype::*;

/// Attempts to run the traveling salesperson problem, using the file named
/// by the argument to populate the map.
/// Returns an Error if the string does not represent a file, or if the file
/// is not correctly formatted.
pub fn run(file_name: &str) -> Result<(), &'static str> {
    let mut rng = rand::thread_rng();

    let point_vector = file_parser::parse_file(file_name)?;
    let graph = WeightedGraph::from_points(point_vector);
    
    let pop_size = 100; 
    let num_alleles = graph.num_vertices();

    // INVARIANT: POPULATION MUST ALWAYS BE SORTED BY FITNESS AT ALL TIMES!!!
    let pop_and_fitness = initial_population(
        pop_size, num_alleles, &graph, &mut rng
    );

    // Reversed because smallest fitness in this problem has the best rank.
    let selection_dist = (0..pop_size)
        .rev()
        .map(|rank| probability_by_rank(2.0, rank, pop_size))
        .collect::<Vec<f64>>();
    let selection_dist = WeightedIndex::new(&selection_dist).unwrap();

    for _ in 0..1 {
        let parent1 = &pop_and_fitness[selection_dist.sample(&mut rng)];
        let parent2 = &pop_and_fitness[selection_dist.sample(&mut rng)];
        
        let child = Genotype::edge_crossover(&parent1.0, &parent2.0, &mut rng);
        println!("{:?} + {:?} = {:?}", parent1.0.data(), parent2.0.data(), child.data());
    }

    Ok(())
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
