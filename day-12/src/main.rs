use std::collections::HashMap;

use anyhow::{Result, anyhow};
use pathfinding::prelude::count_paths;

use cli::{Part, get_part};

fn main() {
    match get_part("inputs/day-12.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{error:?}"),
    }
}

fn part_1(input: &str) -> Result<usize> {
    let connections = parse_input_into_connections(input)?;

    Ok(count_paths(
        Node {
            label: "start",
            double_visit_used: false,
            visited_small_caves: vec!["start"],
        },
        |node| successors(node, &connections),
        |node| node.label == "end",
    ))
}

fn part_2(input: &str) -> Result<usize> {
    let connections = parse_input_into_connections(input)?;

    Ok(count_paths(
        Node {
            label: "start",
            double_visit_used: false,
            visited_small_caves: vec!["start"],
        },
        |node| successors_with_double_visit(node, &connections),
        |node| node.label == "end",
    ))
}

fn parse_input_into_connections(input: &str) -> Result<HashMap<&str, Vec<&str>>> {
    let mut connections: HashMap<&str, Vec<&str>> = HashMap::new();

    input.lines().try_for_each(|line| {
        let Some((a, b)) = line.split_once("-") else {
            return Err(anyhow!("Cannot split line: {}", line));
        };

        connections.entry(a).or_default().push(b);
        connections.entry(b).or_default().push(a);

        Ok(())
    })?;

    Ok(connections)
}

#[derive(Eq, Hash, PartialEq)]
struct Node<'a> {
    label: &'a str,
    double_visit_used: bool,
    visited_small_caves: Vec<&'a str>,
}

fn successors<'a>(node: &Node<'a>, connections: &HashMap<&'a str, Vec<&'a str>>) -> Vec<Node<'a>> {
    let Node {
        label,
        visited_small_caves,
        ..
    } = node;

    let mut nodes = Vec::new();

    for connection in &connections[label] {
        if connection.starts_with(|c: char| c.is_ascii_uppercase()) {
            // Big cave.

            nodes.push(Node {
                label: connection,
                double_visit_used: false,
                visited_small_caves: visited_small_caves.clone(),
            });
        } else if !visited_small_caves.contains(connection) {
            // Unvisited small cave.

            let mut visited_small_caves = visited_small_caves.clone();
            visited_small_caves.push(connection);

            nodes.push(Node {
                label: connection,
                double_visit_used: false,
                visited_small_caves,
            });
        }
    }

    nodes
}

fn successors_with_double_visit<'a>(
    node: &Node<'a>,
    connections: &HashMap<&'a str, Vec<&'a str>>,
) -> Vec<Node<'a>> {
    let Node {
        label,
        double_visit_used,
        visited_small_caves,
    } = node;

    let mut nodes = Vec::new();

    for connection in &connections[label] {
        if connection.starts_with(|c: char| c.is_ascii_uppercase()) {
            // Big cave.

            nodes.push(Node {
                label: connection,
                double_visit_used: *double_visit_used,
                visited_small_caves: visited_small_caves.clone(),
            });
        } else if !visited_small_caves.contains(connection) {
            // Unvisited small cave.

            let mut visited_small_caves = visited_small_caves.clone();
            visited_small_caves.push(connection);

            nodes.push(Node {
                label: connection,
                double_visit_used: *double_visit_used,
                visited_small_caves,
            });
        } else if !*double_visit_used && *connection != "start" {
            // Visited small cave, but double-visit available.

            nodes.push(Node {
                label: connection,
                double_visit_used: true,
                visited_small_caves: visited_small_caves.clone(),
            });
        }
    }

    nodes
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    const EXAMPLE_INPUT_A: &str = r"
start-A
start-b
A-c
A-b
b-d
A-end
b-end
";

    const EXAMPLE_INPUT_B: &str = r"
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
";

    const EXAMPLE_INPUT_C: &str = r"
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
";

    #[test]
    fn example_1a() -> Result<()> {
        assert_eq!(part_1(trim_newlines(EXAMPLE_INPUT_A))?, 10);

        Ok(())
    }

    #[test]
    fn example_1b() -> Result<()> {
        assert_eq!(part_1(trim_newlines(EXAMPLE_INPUT_B))?, 19);

        Ok(())
    }

    #[test]
    fn example_1c() -> Result<()> {
        assert_eq!(part_1(trim_newlines(EXAMPLE_INPUT_C))?, 226);

        Ok(())
    }

    #[test]
    fn example_2a() -> Result<()> {
        assert_eq!(part_2(trim_newlines(EXAMPLE_INPUT_A))?, 36);

        Ok(())
    }

    #[test]
    fn example_2b() -> Result<()> {
        assert_eq!(part_2(trim_newlines(EXAMPLE_INPUT_B))?, 103);

        Ok(())
    }

    #[test]
    fn example_2c() -> Result<()> {
        assert_eq!(part_2(trim_newlines(EXAMPLE_INPUT_C))?, 3509);

        Ok(())
    }
}
