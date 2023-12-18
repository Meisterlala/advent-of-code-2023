crate::solution!(17, solve_a, solve_b);

use std::collections::{BinaryHeap, HashMap};

pub fn solve_a(input: &str) -> u32 {
    let graph = parse(input);
    let goal = (graph.len() - 1, graph[0].len() - 1);
    dijkstra(&graph, (0, 0), goal, filter_edge_a).expect("no path found")
}

pub fn solve_b(input: &str) -> u32 {
    let graph = parse(input);
    let goal = (graph.len() - 1, graph[0].len() - 1);
    dijkstra(&graph, (0, 0), goal, filter_edge_b).expect("no path found")
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Eq, PartialEq, Debug)]
struct Node {
    location: (usize, usize),
    direction: Direction,
    direction_count: u16,
    cost: u32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.cost.cmp(&self.cost))
    }
}

struct Edge {
    direction: Direction,
    weight: u16,
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .map(|c| c as u8)
                .collect()
        })
        .collect()
}

fn dijkstra(
    graph: &[Vec<u8>],
    start: (usize, usize),
    goal: (usize, usize),
    edge_filter: fn(&Edge, &Node) -> bool,
) -> Option<u32> {
    // Distance to the node, with the specified direction and direction count
    let mut dist = HashMap::new();
    dist.insert((start, Direction::East, 0), 1);
    dist.insert((start, Direction::South, 0), 1);

    // Next Nodes we need to look at
    let mut heap = BinaryHeap::new();
    heap.push(Node {
        location: start,
        direction: Direction::East,
        direction_count: 1,
        cost: 0,
    });
    heap.push(Node {
        location: start,
        direction: Direction::South,
        direction_count: 1,
        cost: 0,
    });

    // Examine the Possible Node with min heap
    while let Some(node) = heap.pop() {
        // If we reached the goal, we are done
        if node.location == goal {
            return Some(node.cost);
        }

        // If there already is a better way, we don't need to look at this node
        if let Some(dist) = dist.get(&(node.location, node.direction, node.direction_count)) {
            if node.cost > *dist {
                continue;
            }
        }

        // For each node we can reach, see if we can find a way with a lower cost going through this node
        for edge in valid_edges(graph, &node) {
            // Check for consecutive directions
            if edge_filter(&edge, &node) {
                continue;
            }

            // Calculate the next node
            let next = Node {
                cost: node.cost + edge.weight as u32,
                direction: edge.direction,
                direction_count: if edge.direction == node.direction {
                    node.direction_count + 1
                } else {
                    1
                },
                location: match edge.direction {
                    Direction::North => (node.location.0 - 1, node.location.1),
                    Direction::East => (node.location.0, node.location.1 + 1),
                    Direction::South => (node.location.0 + 1, node.location.1),
                    Direction::West => (node.location.0, node.location.1 - 1),
                },
            };

            // If there already is a easier way to get to this node, we don't need to look at it
            if let Some(dist) = dist.get(&(next.location, next.direction, next.direction_count)) {
                if next.cost >= *dist {
                    continue;
                }
            }

            // Save the cost to get to this node
            dist.insert(
                (next.location, next.direction, next.direction_count),
                next.cost,
            );
            // Look at the next node later
            heap.push(next);
        }
    }

    None
}

fn valid_edges(graph: &[Vec<u8>], node: &Node) -> Vec<Edge> {
    let (x, y) = node.location;
    let mut edges = Vec::new();
    if x > 0 && node.direction != Direction::South {
        edges.push(Edge {
            direction: Direction::North,
            weight: graph[x - 1][y] as u16,
        });
    }
    if y > 0 && node.direction != Direction::East {
        edges.push(Edge {
            direction: Direction::West,
            weight: graph[x][y - 1] as u16,
        });
    }
    if x < graph.len() - 1 && node.direction != Direction::North {
        edges.push(Edge {
            direction: Direction::South,
            weight: graph[x + 1][y] as u16,
        });
    }
    if y < graph[0].len() - 1 && node.direction != Direction::West {
        edges.push(Edge {
            direction: Direction::East,
            weight: graph[x][y + 1] as u16,
        });
    }

    edges
}

fn filter_edge_a(edge: &Edge, node: &Node) -> bool {
    edge.direction == node.direction && node.direction_count >= 3
}

fn filter_edge_b(edge: &Edge, node: &Node) -> bool {
    if edge.direction == node.direction {
        node.direction_count >= 10
    } else {
        node.direction_count < 4
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn example_a() {
        assert_eq!(solve_a(EXAMPLE), 102);
    }

    #[test]
    fn example_b() {
        assert_eq!(solve_b(EXAMPLE), 94);
    }
}
