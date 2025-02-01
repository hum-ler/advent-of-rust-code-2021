use anyhow::{anyhow, Result};
use pathfinding::prelude::dijkstra;

use aoc_cli::{get_part, Part};

fn main() {
    match get_part("inputs/day-15.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: &str) -> Result<u32> {
    let (grid, grid_size) = parse_input_into_grid(input);

    let Some((_, risk)) = dijkstra(
        &(0, 0),
        |node| successors(node, &grid, &grid_size),
        |node| *node == (grid_size.0 - 1, grid_size.1 - 1),
    ) else {
        return Err(anyhow!("Cannot find cheapest path"));
    };

    Ok(risk)
}

fn part_2(input: &str) -> Result<u32> {
    let (grid, grid_size) = parse_input_into_grid(input);

    let grid = expand_grid(grid, 5);
    let grid_size = (grid_size.0 * 5, grid_size.1 * 5);

    let Some((_, risk)) = dijkstra(
        &(0, 0),
        |node| successors(node, &grid, &grid_size),
        |node| *node == (grid_size.0 - 1, grid_size.1 - 1),
    ) else {
        return Err(anyhow!("Cannot find cheapest path"));
    };

    Ok(risk)
}

type Coord = (usize, usize);

type GridSize = (usize, usize);

fn parse_input_into_grid(input: &str) -> (Vec<Vec<u8>>, GridSize) {
    let grid = input
        .lines()
        .map(|line| line.bytes().map(|byte| byte - b'0').collect())
        .collect::<Vec<_>>();

    let row_count = grid.len();
    let col_count = grid.first().map_or(0, Vec::len);

    (grid, (row_count, col_count))
}

fn successors(node: &Coord, grid: &[Vec<u8>], grid_size: &GridSize) -> Vec<(Coord, u32)> {
    let mut nodes = Vec::new();

    if node.0 > 0 {
        nodes.push(((node.0 - 1, node.1), grid[node.0 - 1][node.1] as u32));
    }

    if node.1 < grid_size.1 - 1 {
        nodes.push(((node.0, node.1 + 1), grid[node.0][node.1 + 1] as u32));
    }

    if node.0 < grid_size.0 - 1 {
        nodes.push(((node.0 + 1, node.1), grid[node.0 + 1][node.1] as u32));
    }

    if node.1 > 0 {
        nodes.push(((node.0, node.1 - 1), grid[node.0][node.1 - 1] as u32));
    }

    nodes
}

fn expand_grid(mut tile: Vec<Vec<u8>>, factor: u8) -> Vec<Vec<u8>> {
    // Expand columns.

    tile = tile
        .into_iter()
        .map(|mut row| {
            let mut copy = row.clone();
            for _ in 0..factor {
                copy = copy.into_iter().map(wrapping_increment).collect();

                row.extend(copy.clone());
            }

            row
        })
        .collect();

    // Expand rows.

    let mut copy = tile.clone();
    for _ in 0..factor {
        copy = copy
            .into_iter()
            .map(|row| row.into_iter().map(wrapping_increment).collect())
            .collect();

        tile.extend(copy.clone());
    }

    tile
}

fn wrapping_increment(number: u8) -> u8 {
    if number == 9 {
        1
    } else {
        number + 1
    }
}

#[cfg(test)]
mod tests {
    use aoc_cli::trim_input;

    use super::*;

    const EXAMPLE_INPUT: &str = r"
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(trim_input(EXAMPLE_INPUT))?, 40);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(trim_input(EXAMPLE_INPUT))?, 315);

        Ok(())
    }
}
