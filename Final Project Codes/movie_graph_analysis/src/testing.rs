#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::Graph;
    use crate::analysis::{compute_shortest_paths, compute_centrality, identify_clusters};

    // Helper function to create a simple test graph
    fn create_test_graph() -> Graph {
        let mut graph = Graph::new();
        graph.add_node(1, "Movie A".to_string());
        graph.add_node(2, "Movie B".to_string());
        graph.add_node(3, "Movie C".to_string());
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::Graph;
    use crate::analysis::{compute_shortest_paths, compute_centrality, identify_clusters};

    // Helper function to create a simple test graph
    fn create_test_graph() -> Graph {
        let mut graph = Graph::new();
        graph.add_node(1, "Movie A".to_string());
        graph.add_node(2, "Movie B".to_string());
        graph.add_node(3, "Movie C".to_string());
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph
    }

    #[test]
    fn test_add_nodes_and_edges() {
        let graph = create_test_graph();
        assert!(graph.contains_node(1));
        assert!(graph.contains_node(2));
        assert!(graph.contains_node(3));
        assert_eq!(graph.num_nodes(), 3);
        assert_eq!(graph.num_edges(), 2);
    }

    #[test]
    fn test_compute_shortest_paths() {
        let graph = create_test_graph();
        let paths = compute_shortest_paths(&graph, 1);
        assert_eq!(paths[&2], 1);
        assert_eq!(paths[&3], 2);
    }

    #[test]
    fn test_compute_centrality() {
        let graph = create_test_graph();
        let centrality = compute_centrality(&graph);
        assert!(centrality[&1] > 0.0);
        assert!(centrality[&2] > centrality[&1]);
        assert!(centrality[&3] > 0.0);
    }

    #[test]
    fn test_identify_clusters() {
        let graph = create_test_graph();
        let clusters = identify_clusters(&graph);
        assert_eq!(clusters.len(), 1); // All nodes are connected in this simple graph
        assert!(clusters[0].contains(&1));
        assert!(clusters[0].contains(&2));
        assert!(clusters[0].contains(&3));
    }
}
