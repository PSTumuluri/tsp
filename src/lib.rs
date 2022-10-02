mod file_parser;
mod weighted_graph;
mod genotype;

use rand::prelude::*;

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
    
    let num_cities = graph.num_vertices();
    let pop_size = 100;
    let population = (0..100)
        .map(|_| Genotype::random(num_cities, &mut rng))
        .collect::<Vec<Genotype>>();

    Ok(())
}

pub fn fitness(environ: WeightedGraph, individual: &[usize]) -> f64 {
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
