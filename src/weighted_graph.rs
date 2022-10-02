mod utils;

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
                matrix[utils::flat_index(num_vertices, i, j)] = 
                    utils::distance(point_vec[i], point_vec[j]);
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

    pub fn print(&self) {
        for i in 0..self.num_vertices {
            println!("{:?}", &self.matrix[i*self.num_vertices..(i+1)*self.num_vertices]);
        }
    }
}
