use petgraph::algo::dijkstra;
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use std::collections::HashMap;
use std::collections::HashSet;
use graph_construction::GameGraph;
use crate::graph_construction;

// For a given node, compute shortest paths to all other nodes
fn compute_all_pairs_shortest_paths(graph: &Graph<String, u32>) -> HashMap<NodeIndex, HashMap<NodeIndex, u32>> {
    let mut all_paths = HashMap::new();
    for node in graph.node_indices() {
        let paths = dijkstra(graph, node, None, |e| *e.weight());
        all_paths.insert(node, paths);
    }
    all_paths
}


/// Finds the densest subgraph in the given graph.
pub fn find_densest_subgraph(graph: &GameGraph) -> (HashSet<String>, f64) {
    let mut max_density = 0.0;
    let mut best_subgraph = HashSet::new();

    // Naive approach: Consider all possible subgraphs (computationally expensive)
    // For demonstration, we simplify by examining subgraphs generated by removing one node at a time
    for node in graph.graph.node_indices() {
        let mut subgraph = Graph::<String, u32>::new();
        let mut index_map = HashMap::new();

        // Rebuild the graph without the current node
        for other_node in graph.graph.node_indices() {
            if other_node != node {
                let node_name = &graph.graph[other_node];
                let index = subgraph.add_node(node_name.clone());
                index_map.insert(node_name.clone(), index);
            }
        }

        // Reconnect edges excluding the removed node
        for edge in graph.graph.raw_edges() {
            if edge.source() != node && edge.target() != node {
                let source_name = &graph.graph[edge.source()];
                let target_name = &graph.graph[edge.target()];
                subgraph.add_edge(
                    *index_map.get(source_name).unwrap(),
                    *index_map.get(target_name).unwrap(),
                    edge.weight,
                );
            }
        }

        // Calculate density of this subgraph
        let num_edges = subgraph.edge_count();
        let num_nodes = subgraph.node_count();
        if num_nodes > 1 {
            let density = num_edges as f64 / num_nodes as f64;
            if density > max_density {
                max_density = density;
                best_subgraph = subgraph.node_indices().map(|i| subgraph[i].clone()).collect();
            }
        }
    }

    (best_subgraph, max_density)
}

