use std::collections::HashMap;
use crate::graph_construction::GameGraph; // Import the GameGraph from graph_construction.rs

/// Computes the degree of each vertex in the graph and returns a HashMap of vertex indices to their degrees.
pub fn compute_degrees(graph: &GameGraph) -> HashMap<String, usize> {
    let mut degrees = HashMap::new();

    for node in graph.graph.node_indices() {
        let degree = graph.graph.edges_directed(node, petgraph::Direction::Outgoing).count();
        let platform_name = &graph.graph[node];
        degrees.insert(platform_name.clone(), degree);
    }

    degrees
}

/// Analyzes the degree distribution and prints a summary.
pub fn analyze_degree_distribution(degrees: &HashMap<String, usize>) {
    let mut degree_counts = HashMap::new();

    for &degree in degrees.values() {
        *degree_counts.entry(degree).or_insert(0) += 1;
    }

    println!("Degree Distribution:");
    for (degree, count) in degree_counts.iter() {
        println!("Degree {}: {} platforms", degree, count);
    }
}

