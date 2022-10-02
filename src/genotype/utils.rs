use super::*;

use std::collections::HashMap;
use rand::thread_rng;

pub fn construct_edge_table(parent1: &[usize], parent2: &[usize]) 
-> Vec<HashMap<usize, usize>> {
    let num_alleles = parent1.len();

    let mut edge_table = vec![HashMap::with_capacity(4); num_alleles];

    for i in 0..num_alleles-1 {
        add_to_edge_table(&mut edge_table, parent1[i], parent1[i+1]);
        add_to_edge_table(&mut edge_table, parent2[i], parent2[i+1]);
    }

    // Special case: 0 is adjacent to N-1
    add_to_edge_table(&mut edge_table, parent1[0], parent1[num_alleles-1]);
    add_to_edge_table(&mut edge_table, parent2[0], parent2[num_alleles-1]);
       
    edge_table
}

/// Inserts an edge into the edge table, increasing its multiplicity count if
/// the edge has already been inserted, or initializing it to 1 if it hasn't.
fn add_to_edge_table(edge_table: &mut Vec<HashMap<usize, usize>>, 
                     v: usize, u: usize) {
    let count = edge_table[v].entry(u).or_insert(0);
    *count += 1;

    let count = edge_table[u].entry(v).or_insert(0);
    *count += 1;
}

/// Attempt to select a vertex adjacent to the current one from the edge table.
/// First tries to find an adjacent vertex common to both parents. Then tries
/// to find any adjacent vertex which itself has the smallest non-zero length
/// edge list. If it finds only empty lists, it returns None.
pub fn try_select_adjacent(edge_table: &Vec<HashMap<usize, usize>>, 
                        vertex: usize, rng: &mut ThreadRng) -> Option<usize> {
    let row = &edge_table[vertex];

    // Attempt 1: Try to find edge common to both parents.
    for (&adjacent_vertex, &count) in row.iter() {
        if count == 2 {
            return Some(adjacent_vertex);
        }
    }

    // Attempt 2: Try to find the adjcent vertex with the smallest edge list.
    let min_list_len = row
        .keys()
        .map(|&v| edge_table[v].len())
        .min()
        .expect("really no way for this to happen unless the the whole table is somehow empty which shouldn't be allowed anyway so...");
    
    if min_list_len != 0 {
        let min_list_vertices = row
            .keys()
            .copied()
            .filter(|&v| edge_table[v].len() == min_list_len)
            .collect::<Vec<usize>>();
        
        return Some(*min_list_vertices.choose(rng).unwrap());
    }

    None
}

/// Removes the specified vertex from the adjacency lists of each vertex.
/// Does NOT remove the vertex as an index into the table, that is, vertices
/// adjacent to it can still be found.
pub fn remove_edge(edge_table: &mut Vec<HashMap<usize, usize>>, vertex: usize) {
    for list in edge_table.iter_mut() {
        list.remove(&vertex);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edge_table_is_correctly_constructed() {
        let mut rng = rand::thread_rng();

        // All edges in permutations of size three are adjacent to each other.
        let parent1 = vec![0, 1, 2];
        let parent2 = vec![2, 1, 0];

        let mut edge_table = construct_edge_table(&parent1, &parent2);

        assert_eq!(2, edge_table[0].len());
        assert_eq!(2, *edge_table[0].get(&1).unwrap());
        assert_eq!(2, *edge_table[0].get(&2).unwrap());

        // Here, 0 is adjacent to all four other vertices, once each.
        let parent1 = vec![0, 1, 2, 3, 4];
        let parent2 = vec![0, 2, 4, 1, 3];

        let edge_table = construct_edge_table(&parent1, &parent2);
        assert_eq!(4, edge_table[0].len());
        assert_eq!(1, *edge_table[0].get(&1).unwrap());
        assert_eq!(1, *edge_table[0].get(&2).unwrap());
        assert_eq!(1, *edge_table[0].get(&3).unwrap());
        assert_eq!(1, *edge_table[0].get(&4).unwrap());
    }
}
