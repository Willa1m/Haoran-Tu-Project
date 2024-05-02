use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::{VisitMap, Visitable};
use std::collections::{HashMap, VecDeque};
use crate::graph_construction::GameGraph; // Import the GameGraph from graph_construction.rs

/// Calculate the average shortest path length between all pairs of nodes using BFS.
pub fn average_path_length(graph: &GameGraph) -> f64 {
    let mut total_distance = 0.0;
    let mut count = 0;

    for start_index in graph.graph.node_indices() {
        let distances = bfs_shortest_paths(&graph.graph, start_index);
        for (&node_index, &dist) in &distances {
            if node_index != start_index {
                total_distance += dist as f64;
                count += 1;
            }
        }
    }

    if count > 0 {
        total_distance / count as f64
    } else {
        0.0
    }
}

/// Performs BFS to find the shortest path distances from a starting node to all other nodes.
fn bfs_shortest_paths(graph: &DiGraph<String, u32>, start: NodeIndex) -> HashMap<NodeIndex, usize> {
    let mut distances = HashMap::new();
    let mut visit_queue = VecDeque::new();
    let mut visited = graph.visit_map();

    distances.insert(start, 0);
    visit_queue.push_back(start);
    visited.visit(start);

    while let Some(node) = visit_queue.pop_front() {
        let current_distance = *distances.get(&node).unwrap();

        for neighbor in graph.neighbors(node) {
            if !visited.is_visited(&neighbor) {
                visited.visit(neighbor);
                distances.insert(neighbor, current_distance + 1);
                visit_queue.push_back(neighbor);
            }
        }
    }

    distances
}

/// use the average path length function.
pub fn function_usage(game_graph: &GameGraph) {
    let avg_path_length = average_path_length(game_graph);
    println!("Average path length in the graph: {}", avg_path_length);
}
