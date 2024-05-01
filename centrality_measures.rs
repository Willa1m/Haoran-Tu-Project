use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::algo::dijkstra;
use std::collections::HashMap;
use crate::graph_construction::GameGraph;

/// Calculates the closeness centrality for each node in the graph.
pub fn closeness_centrality(graph: &GameGraph) -> HashMap<String, f64> {
    let mut centralities = HashMap::new();
    let node_indices: Vec<NodeIndex> = graph.graph.node_indices().collect();
    let node_count = node_indices.len() as f64;

    for &node in &node_indices {
        let distances = dijkstra(&graph.graph, node, None, |e| *e.weight());
        let total_distance: usize = distances.values()
        .filter_map(|cost_option| Some(cost_option))  // Filter out None and unwrap Some
        .map(|cost| *cost as usize)  // Convert u32 to usize
        .sum();
 
        let closeness = if total_distance > 0 {
            (node_count - 1.0) / total_distance as f64
        } else {
            0.0
        };

        if let Some(label) = graph.graph.node_weight(node).cloned() {
            centralities.insert(label, closeness);
        }
    }

    centralities
}

/// Example function to show how to use the closeness centrality function.
pub fn example_usage(game_graph: &GameGraph) {
    let centralities = closeness_centrality(game_graph);
    for (platform, centrality) in centralities {
        println!("Closeness Centrality of {}: {}", platform, centrality);
    }
}
