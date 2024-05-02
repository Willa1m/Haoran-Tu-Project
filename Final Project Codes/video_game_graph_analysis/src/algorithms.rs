use petgraph::graph::NodeIndex;
use petgraph::visit::{Bfs, Dfs, Visitable, VisitMap};
use crate::graph_construction::GameGraph; // Import the GameGraph from graph_construction.rs

/// Performs a Breadth-First Search (BFS) starting from a given node and applies a function to each visited node.
pub fn breadth_first_search<F>(graph: &GameGraph, start_index: NodeIndex, mut visit_fn: F)
where
    F: FnMut(NodeIndex) {
    let mut bfs = Bfs::new(&graph.graph, start_index);
    let mut visited = graph.graph.visit_map();

    while let Some(nx) = bfs.next(&graph.graph) {
        if visited.visit(nx) {
            visit_fn(nx);
        }
    }
}

/// Performs a Depth-First Search (DFS) starting from a given node and applies a function to each visited node.
pub fn depth_first_search<F>(graph: &GameGraph, start_index: NodeIndex, mut visit_fn: F)
where
    F: FnMut(NodeIndex) {
    let mut dfs = Dfs::new(&graph.graph, start_index);
    let mut visited = graph.graph.visit_map();

    while let Some(nx) = dfs.next(&graph.graph) {
        if visited.visit(nx) {
            visit_fn(nx);
        }
    }
}
