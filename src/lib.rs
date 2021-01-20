//! This is a DIMACS "graph file" reader, following roughly
//! the DIMACS Graph File
//! [specification](https://mat.tepper.cmu.edu/COLOR/general/ccformat.ps).

use std::collections::{HashSet, HashMap};
use std::io::{Read, BufRead, BufReader};

/// Graphs are represented in adjacency list format.
pub type Graph = HashMap<u64, HashSet<u64>>;

/// Takes a readable ASCII DIMACS representation.  Returns
/// the graph.
pub fn read_graph<R: Read>(graph_file: R) -> Graph {
    let mut result = HashMap::new();
    let mut adj = |n1, n2| {
        let ns = result.entry(n1).or_insert_with(HashSet::new);
        ns.insert(n2);
    };

    let mut lines = BufReader::new(graph_file)
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| l.len() > 1 && l.chars().next() != Some('c'));
    let header_line = lines.next().unwrap();
    let header: Vec<&str> =  header_line.split_whitespace().collect();
    assert_eq!(4, header.len());
    assert_eq!(header[0], "p");
    assert_eq!(header[1], "edge");
    let nnodes: usize = header[2].parse().unwrap();
    let nedges: usize = header[3].parse().unwrap();
    let mut nedges_seen = 0;
    for line in lines {
        let edge: Vec<&str> = line
            .split_whitespace()
            .collect();
        assert_eq!(3, edge.len());
        assert_eq!("e", edge[0]);
        let n1: u64 = edge[1].parse().unwrap();
        let n2: u64 = edge[2].parse().unwrap();
        adj(n1, n2);
        adj(n2, n1);
        nedges_seen += 1;
    }
    // XXX Some graphs count two edges per undirected edge,
    // some count one.
    assert!(nedges == nedges_seen || 2 * nedges == nedges_seen);
    assert!(nnodes >= result.len());
    result
}
