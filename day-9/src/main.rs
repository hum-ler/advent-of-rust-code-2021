use anyhow::Result;

use cli::{Part, get_part};

fn main() {
    match get_part("inputs/day-9.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{error:?}"),
    }
}

fn part_1(input: &str) -> Result<u32> {
    let (heightmap, grid_size) = parse_input_into_heightmap(input)?;

    Ok((0..grid_size.0)
        .map(|row| {
            (0..grid_size.1)
                .map(|col| {
                    if is_low_point((row, col), &heightmap, &grid_size) {
                        (heightmap[row][col] - b'0' + 1) as u32
                    } else {
                        0
                    }
                })
                .sum::<u32>()
        })
        .sum())
}

fn part_2(input: &str) -> Result<usize> {
    let (mut heightmap, grid_size) = parse_input_into_heightmap(input)?;

    let mut basin_sizes = (0..grid_size.0)
        .flat_map(|row| {
            (0..grid_size.1)
                .filter_map(|col| {
                    if is_low_point((row, col), &heightmap, &grid_size) {
                        Some(flood_basin((row, col), &mut heightmap, &grid_size))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    basin_sizes.sort_by(|a, b| b.cmp(a));

    Ok(basin_sizes[0] * basin_sizes[1] * basin_sizes[2])
}

type Coord = (usize, usize);

type GridSize = (usize, usize);

fn parse_input_into_heightmap(input: &str) -> Result<(Vec<Vec<u8>>, GridSize)> {
    let heightmap = input
        .lines()
        .map(|line| line.bytes().collect())
        .collect::<Vec<_>>();

    let row_count = heightmap.len();
    let col_count = heightmap.first().map_or(0, Vec::len);

    Ok((heightmap, (row_count, col_count)))
}

fn is_low_point(pos: Coord, heightmap: &[Vec<u8>], grid_size: &GridSize) -> bool {
    let mut neighbours = [u8::MAX; 4];

    if pos.0 > 0 {
        neighbours[0] = heightmap[pos.0 - 1][pos.1];
    }

    if pos.1 < grid_size.1 - 1 {
        neighbours[1] = heightmap[pos.0][pos.1 + 1];
    }

    if pos.0 < grid_size.0 - 1 {
        neighbours[2] = heightmap[pos.0 + 1][pos.1];
    }

    if pos.1 > 0 {
        neighbours[3] = heightmap[pos.0][pos.1 - 1];
    }

    neighbours
        .iter()
        .all(|neighbour| *neighbour > heightmap[pos.0][pos.1])
}

fn flood_basin(low_point: Coord, heightmap: &mut [Vec<u8>], grid_size: &GridSize) -> usize {
    // Termination case.
    if matches!(heightmap[low_point.0][low_point.1], b'9' | u8::MAX) {
        return 0;
    }

    // Mark the position as done.
    heightmap[low_point.0][low_point.1] = u8::MAX;

    let mut neighbours = Vec::new();

    if low_point.0 > 0 {
        neighbours.push((low_point.0 - 1, low_point.1));
    }

    if low_point.1 < grid_size.1 - 1 {
        neighbours.push((low_point.0, low_point.1 + 1));
    }

    if low_point.0 < grid_size.0 - 1 {
        neighbours.push((low_point.0 + 1, low_point.1));
    }

    if low_point.1 > 0 {
        neighbours.push((low_point.0, low_point.1 - 1));
    }

    1 + neighbours
        .into_iter()
        .map(|neighbour| flood_basin(neighbour, heightmap, grid_size))
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    const EXAMPLE_INPUT: &str = r"
2199943210
3987894921
9856789892
8767896789
9899965678
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(trim_newlines(EXAMPLE_INPUT))?, 15);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(trim_newlines(EXAMPLE_INPUT))?, 1134);

        Ok(())
    }
}
