mod utils;

use std::mem::swap;

pub struct WeightedGraph {
    matrix: Vec<f64>,
    num_vertices: usize,
}

impl WeightedGraph {

    /// Converts the specified set of points on the xy-plane into a complete 
    /// weighted graph.
    /// That is, each vertex is connected to every other vertex, and the weight
    /// of the edge connecting them is the distance between the two points on
    /// the xy-plane.
    pub fn from_points(point_vec: Vec<(f64, f64)>) -> Self {

        // NOTE THAT THIS IMPLEMENTATION SUCKS AND NEEDS TO BE CHANGED
        // Since the adjacency matrix of a complete graph is obviously 
        // symmetric, this matrix is upper-triangular with the main diagonal
        // also being all 0. Therefore we only need to store 
        // (n-1) + (n-2) + ... + 1 elements. However, this also makes the
        // indexing scheme more complicated. So in order to get this project
        // done I am leaving it like this, knowing that it can be converted to
        // the better form using basic arithmetic when time permits.

        let num_vertices = point_vec.len();
        let mut matrix = vec![0.0; num_vertices.pow(2)];
        
        for i in 0..num_vertices {
            for j in i+1..num_vertices {
                let distance = utils::distance(point_vec[i], point_vec[j]);
                matrix[Self::flat_index(num_vertices, i, j)] = distance;
                matrix[Self::flat_index(num_vertices, j, i)] = distance;
            }
        }
        
        WeightedGraph {
            matrix,
            num_vertices,
        }
    }

    pub fn num_vertices(&self) -> usize {
        self.num_vertices
    }

    /// Returns the label on the edge between vertices v and u.
    pub fn weight_between(&self, mut v: usize, mut u: usize) -> f64 {
        self.matrix[Self::flat_index(self.num_vertices, v, u)]
    }

    pub fn print(&self) {
        for i in 0..self.num_vertices {
            println!(
                "{:?}", 
                &self.matrix[i*self.num_vertices..(i+1)*self.num_vertices]
            );
        }
    }

    /// Converts a 2D index in the conceptual matrix to a 1D index in the flat
    /// matrix representation.
    fn flat_index(num_vertices: usize, i: usize, j: usize) -> usize {
        i * num_vertices + j
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weight_between_vertices_is_computed_correctly() {
        let graph = WeightedGraph {
            num_vertices: 3,
            matrix: vec![
                1.0, 2.0, 3.0,
                2.0, 1.0, 4.0,
                3.0, 4.0, 1.0,
            ],
        };
        assert_eq!(4.0, graph.weight_between(2, 1));
        assert_eq!(4.0, graph.weight_between(1, 2));

        assert_eq!(2.0, graph.weight_between(0, 1));
        assert_eq!(2.0, graph.weight_between(1, 0));
    }
}
