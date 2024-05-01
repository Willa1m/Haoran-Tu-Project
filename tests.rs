#[cfg(test)]
mod tests {
    // Import everything from the outer module
    use crate::GameGraph;

    #[test]
    fn test_add_platform() {
        let mut graph = GameGraph::new();
        let index = graph.add_platform("PC");
        assert!(graph.graph.node_weight(index).is_some());
        assert_eq!(graph.graph.node_weight(index).unwrap(), "PC");
    }

    #[test]
    fn test_add_release_edge() {
        let mut graph = GameGraph::new();
        let from_index = graph.add_platform("PC");
        let to_index = graph.add_platform("Xbox");
        graph.add_release_edge("PC", "Xbox");

        assert!(graph.graph.contains_edge(from_index, to_index));
    }
}
