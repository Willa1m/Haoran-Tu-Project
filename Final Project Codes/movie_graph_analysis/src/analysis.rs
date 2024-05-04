use crate::graph::Graph;
use std::collections::{VecDeque, HashMap, HashSet};

// Function to compute the shortest paths using Breadth-First Search (BFS)
pub fn compute_shortest_paths(graph: &Graph, start_node: u32) -> HashMap<u32, u32> {
    let mut distances = HashMap::new();
    let mut queue = VecDeque::new();

    distances.insert(start_node, 0);
    queue.push_back(start_node);

    while let Some(current) = queue.pop_front() {
        let current_distance = distances[&current];

        if let Some(neighbors) = graph.neighbors(current) {
            for &neighbor in neighbors {
                if !distances.contains_key(&neighbor) {
                    queue.push_back(neighbor);
                    distances.insert(neighbor, current_distance + 1);
                }
            }
        }
    }

    distances
}

// Function to compute centrality measures
pub fn compute_centrality(graph: &Graph) -> HashMap<u32, f32> {
    let mut centrality_scores = HashMap::new();

    for &node in graph.nodes.keys() {
        let distances = compute_shortest_paths(graph, node);
        let sum_distances = distances.values().sum::<u32>() as f32;
        let closeness_centrality = if sum_distances > 0.0 {
            1.0 / sum_distances
        } else {
            0.0
        };
        centrality_scores.insert(node, closeness_centrality);
    }

    centrality_scores
}

// Function to identify clusters in the graph (here using a simple community detection placeholder)
pub fn identify_clusters(graph: &Graph) -> Vec<HashSet<u32>> {
    let mut clusters = Vec::new();
    let mut visited = HashSet::new();
    for &node in graph.nodes.keys() {
        if !visited.contains(&node) {
            let cluster = bfs_cluster(graph, node, &mut visited);
            clusters.push(cluster);
        }
    }
    clusters
}

// Helper function for BFS-based cluster detection
fn bfs_cluster(graph: &Graph, start_node: u32, visited: &mut HashSet<u32>) -> HashSet<u32> {
    let mut queue = VecDeque::new();
    let mut cluster = HashSet::new();

    queue.push_back(start_node);
    visited.insert(start_node);
    cluster.insert(start_node);

    while let Some(current) = queue.pop_front() {
        if let Some(neighbors) = graph.neighbors(current) {
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back(neighbor);
                    cluster.insert(neighbor);
                }
            }
        }
    }

    cluster
}

pub fn run_analyses(graph: &Graph) {
    // Compute shortest paths from an arbitrary start node for demonstration; adjust as needed
    if let Some(&start_node) = graph.nodes.keys().next() {
        let shortest_paths = compute_shortest_paths(graph, start_node);
        println!("Shortest paths from node {}: {:?}", start_node, shortest_paths);
    }

    // Compute centrality measures
    let centrality_scores = compute_centrality(graph);
    println!("Centrality scores: {:?}", centrality_scores);

    // Identify clusters
    let clusters = identify_clusters(graph);
    println!("Detected clusters: {:?}", clusters);
}