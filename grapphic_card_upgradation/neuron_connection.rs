use std::collections::{HashMap, HashSet, VecDeque};

// Define a struct for the Graph
#[derive(Debug)]
struct Graph<T> {
    adj_list: HashMap<T, HashSet<T>>,
}

impl<T> Graph<T>
where
    T: Eq + std::hash::Hash + Copy + Debug,
{
    // Create a new empty Graph
    fn new() -> Self {
        Graph {
            adj_list: HashMap::new(),
        }
    }

    // Add a vertex to the graph
    fn add_vertex(&mut self, vertex: T) {
        self.adj_list.entry(vertex).or_insert(HashSet::new());
    }

    // Add an edge to the graph (undirected)
    fn add_edge(&mut self, src: T, dest: T) {
        self.adj_list.entry(src).or_insert(HashSet::new()).insert(dest);
        self.adj_list.entry(dest).or_insert(HashSet::new()).insert(src);
    }

    // Depth-First Search (DFS)
    fn dfs(&self, start: T) -> Vec<T> {
        let mut visited = HashSet::new();
        let mut stack = vec![start];
        let mut result = Vec::new();

        while let Some(node) = stack.pop() {
            if visited.insert(node) {
                result.push(node);
                if let Some(neighbors) = self.adj_list.get(&node) {
                    for &neighbor in neighbors.iter() {
                        stack.push(neighbor);
                    }
                }
            }
        }
        result
    }

    // Breadth-First Search (BFS)
    fn bfs(&self, start: T) -> Vec<T> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut result = Vec::new();

        visited.insert(start);
        queue.push_back(start);

        while let Some(node) = queue.pop_front() {
            result.push(node);

            if let Some(neighbors) = self.adj_list.get(&node) {
                for &neighbor in neighbors.iter() {
                    if visited.insert(neighbor) {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
        result
    }
}

fn main() {
    // Create a new graph
    let mut graph = Graph::new();

    // Add vertices
    graph.add_vertex(1);
    graph.add_vertex(2);
    graph.add_vertex(3);
    graph.add_vertex(4);
    graph.add_vertex(5);

    // Add edges
    graph.add_edge(1, 2);
    graph.add_edge(1, 3);
    graph.add_edge(2, 4);
    graph.add_edge(3, 5);
    graph.add_edge(4, 5);

    // Print the graph
    println!("Graph: {:?}", graph);

    // Perform DFS and BFS
    let dfs_result = graph.dfs(1);
    let bfs_result = graph.bfs(1);

    println!("DFS Result: {:?}", dfs_result);
    println!("BFS Result: {:?}", bfs_result);
}
