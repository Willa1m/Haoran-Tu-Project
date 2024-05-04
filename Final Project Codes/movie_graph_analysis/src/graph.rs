use std::collections::{HashMap, HashSet};
use crate::data::{Movie, Rating, Tag};

pub struct Graph {
    pub nodes: HashMap<u32, String>,  // Maps movie_id to movie title
    pub edges: HashMap<u32, HashSet<u32>>,  // Adjacency list representation of the graph
}

impl Graph {
    // Constructor to create a new Graph instance
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    // Add a node to the graph
    pub fn add_node(&mut self, movie_id: u32, title: String) {
        self.nodes.entry(movie_id).or_insert(title);
    }

    // Add an edge between two nodes in the graph
    pub fn add_edge(&mut self, movie_id1: u32, movie_id2: u32) {
        self.edges.entry(movie_id1).or_insert_with(HashSet::new).insert(movie_id2);
        self.edges.entry(movie_id2).or_insert_with(HashSet::new).insert(movie_id1);
    }

    // Check if the graph contains a node
    pub fn contains_node(&self, movie_id: u32) -> bool {
        self.nodes.contains_key(&movie_id)
    }

    // Get neighbors of a specific node
    pub fn neighbors(&self, movie_id: u32) -> Option<&HashSet<u32>> {
        self.edges.get(&movie_id)
    }

    // Number of nodes in the graph
    pub fn num_nodes(&self) -> usize {
        self.nodes.len()
    }

    // Number of edges in the graph
    pub fn num_edges(&self) -> usize {
        self.edges.values().map(|neighbors| neighbors.len()).sum::<usize>() / 2
    }
}

// This function assumes that you have structures for Movie, Rating, and Tag defined elsewhere
pub fn create_graph(movies: &[Movie], ratings: &[Rating], _tags: &[Tag]) -> Graph {
    let mut graph = Graph::new();

    // Add movies as nodes
    for movie in movies {
        graph.add_node(movie.movie_id, movie.title.clone());
    }

    // Create a map of user to the movies they've rated
    let mut user_movie_map: HashMap<u32, HashSet<u32>> = HashMap::new();
    
    for rating in ratings {
        user_movie_map.entry(rating.user_id).or_default().insert(rating.movie_id);
    }

    // Adding edges based on shared ratings by the same user
    for movies in user_movie_map.values() {
        let movie_list: Vec<u32> = movies.iter().cloned().collect();
        for (i, &movie_id1) in movie_list.iter().enumerate() {
            for &movie_id2 in &movie_list[i+1..] {
                graph.add_edge(movie_id1, movie_id2);
            }
        }
    }

    graph
}

pub fn debug_graph_info(graph: &Graph) {
    println!("Total nodes: {}", graph.num_nodes());
    println!("Total edges: {}", graph.num_edges());

    // Check for the presence of a specific node, assuming a known movie_id
    let movie_id_example = 1;
    if graph.contains_node(movie_id_example) {
        println!("Movie ID {} is present in the graph.", movie_id_example);
    }

    // Optionally, check the connections of a specific node
    if let Some(neighbors) = graph.neighbors(movie_id_example) {
        println!("Neighbors of movie ID {}: {:?}", movie_id_example, neighbors);
        }
    }
