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
