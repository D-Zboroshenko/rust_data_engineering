use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::Direction;
use std::fmt;

#[derive(Debug)]
struct Fighter {
    name: String,
}
/*
This is a bit like the following Python code:

class Fighter:
    def __init__(self, name):
        self.name = name
*/
impl Fighter {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl fmt::Display for Fighter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn add_edge(graph: &mut UnGraph<&Fighter, f32>, nodes: &[NodeIndex], a: usize, b: usize) {
    graph.add_edge(nodes[a], nodes[b], 1.0);
}

// The PageRank struct holds the damping factor and the number of iterations to run the algorithm.
struct PageRank {
    damping: f64,
    iterations: usize,
}

impl PageRank {
    // The new function creates a new instance of the PageRank struct.
    fn new(damping: f64, iterations: usize) -> Self {
        Self {
            damping,
            iterations,
        }
    }

    // The rank function calculates and returns the PageRank for each node in the graph.
    fn rank(&self, graph: &UnGraph<&Fighter, f32>, fighter_nodes: &Vec<NodeIndex>) -> Vec<f64> {
        // The number of nodes in the graph.
        let n = graph.node_count();

        // The initial PageRank value for each node.
        let mut ranks = vec![1.0 / (n as f64); n];

        // Iterates the specified number of times.
        for _ in 0..self.iterations {
            // A new vector to hold the updated PageRank values.
            let mut new_ranks = vec![0.0; n];

            // Iterates over each node and its edges in the graph.
            for (i, &node) in fighter_nodes.iter().enumerate() {
                // The amount of PageRank value this node contributes to its linked nodes.
                let degree = graph.edges_directed(node, Direction::Outgoing).count() as f64;
                let contribution = ranks[i] / degree;

                // Distributes the PageRank value to the linked nodes.
                for neighbor in graph.neighbors(node) {
                    new_ranks[neighbor.index()] += contribution;
                }
            }

            // Updates the PageRank values using the damping factor.
            for rank in &mut new_ranks {
                *rank = *rank * self.damping + (1.0 - self.damping) / (n as f64);
            }

            // Replaces the old PageRank values with the new ones.
            ranks = new_ranks;
        }

        // Returns the final PageRank values.
        ranks
    }
}

fn main() {
    let mut graph = UnGraph::new_undirected();

    let fighters = [
        Fighter::new("Dustin Poirier"),
        Fighter::new("Khabib Nurmagomedov"),
        Fighter::new("Jose Aldo"),
        Fighter::new("Conor McGregor"),
        Fighter::new("Nate Diaz"),
    ];

    let fighter_nodes: Vec<NodeIndex> = fighters
        .iter()
        .map(|fighter| graph.add_node(fighter))
        .collect();

    add_edge(&mut graph, &fighter_nodes, 0, 1); // Dustin Poirier vs. Khabib Nurmagomedov
    add_edge(&mut graph, &fighter_nodes, 1, 3); // Khabib Nurmagomedov vs. Conor McGregor
    add_edge(&mut graph, &fighter_nodes, 3, 0); // Conor McGregor vs. Dustin Poirier
    add_edge(&mut graph, &fighter_nodes, 3, 2); // Conor McGregor vs. Jose Aldo
    add_edge(&mut graph, &fighter_nodes, 3, 4); // Conor McGregor vs. Nate Diaz
    add_edge(&mut graph, &fighter_nodes, 0, 4); // Dustin Poirier vs. Nate Diaz
    add_edge(&mut graph, &fighter_nodes, 2, 4); // Jose Aldo vs. Nate Diaz

    /*
    let node = fighter_nodes[0];
    let neighbors = graph.neighbors(node);
    for i in neighbors {
        println!("{:?}", i.index());
    }
    */

    let pagerank = PageRank::new(0.85, 100);
    // Calculates the PageRank values.
    let ranks = pagerank.rank(&graph, &fighter_nodes);

    // Prints the PageRank values.
    for (i, rank) in ranks.iter().enumerate() {
        println!("The PageRank of {} is {}", fighters[i], rank);
    }
    println!();

    // closeness centrality
    for (i, &node) in fighter_nodes.iter().enumerate() {
        let name = &fighters[i].name;
        let mut degree = 0.0;
        for j in 0..5 {
            if j != i {
                let node2 = fighter_nodes[j];
                let node_map = dijkstra(&graph, node, Some(node2), |e| *e.weight());
                degree += node_map.get(&node2).unwrap();
            }
        }
        let closeness = 4.0 / degree;
        println!("The closeness centrality of {} is {:.2}", name, closeness);
    }

    graph.node_count();

    /*
    for (i, &node) in fighter_nodes.iter().enumerate() {
        let name = &fighters[i].name;
        let degree = graph.edges_directed(node, Direction::Outgoing).count() as f32;
        let closeness = 1.0 / degree;
        println!("The closeness centrality of {} is {:.2}", name, closeness);

        // Explanation
        match name.as_str() {
            "Conor McGregor" => println!(
                "{} has the lowest centrality because he has fought with all other fighters in the network. In this context, a lower centrality value means a higher number of fights.",
                name
            ),
            "Dustin Poirier" | "Nate Diaz" => println!(
                "{} has a centrality of {:.2}, implying they had less fights compared to Conor McGregor but more than Khabib Nurmagomedov and Jose Aldo.",
                name, closeness
            ),
            "Khabib Nurmagomedov" | "Jose Aldo" => println!(
                "{} has the highest centrality of {:.2} as they have fought with the least number of fighters.",
                name, closeness
            ),
            _ => {}
        }
        println!("-----------------");

    }
    */
}
