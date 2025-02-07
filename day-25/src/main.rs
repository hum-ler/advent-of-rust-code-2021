use std::collections::HashMap;

use anyhow::Result;

use aoc_cli::{get_part, Part};

fn main() {
    match get_part("inputs/day-25.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(_)) => println!("No part 2"),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: &str) -> Result<usize> {
    let (mut sea_cucumbers, grid_size) = parse_input_to_grid(input);

    let mut steps = 0;

    while move_sea_cucumbers(&mut sea_cucumbers, &grid_size) > 0 {
        steps += 1;
    }

    Ok(steps + 1)
}

#[derive(Clone)]
enum SeaCucumber {
    East,
    South,
}

type Coord = (usize, usize);

type GridSize = (usize, usize);

fn parse_input_to_grid(input: &str) -> (HashMap<Coord, SeaCucumber>, GridSize) {
    let mut sea_cucumbers = HashMap::new();
    let mut grid_size = (0, 0);

    input.lines().enumerate().for_each(|(row, line)| {
        line.bytes().enumerate().for_each(|(col, byte)| match byte {
            b'>' => {
                sea_cucumbers.entry((row, col)).or_insert(SeaCucumber::East);
            }
            b'v' => {
                sea_cucumbers
                    .entry((row, col))
                    .or_insert(SeaCucumber::South);
            }
            _ => (),
        });

        grid_size.0 = row + 1;
        grid_size.1 = line.len();
    });

    (sea_cucumbers, grid_size)
}

/// Moves sea cucumbers by 1 step according to the rules.
///
/// Returns the number of sea cucumbers that moved in this step.
fn move_sea_cucumbers(
    sea_cucumbers: &mut HashMap<Coord, SeaCucumber>,
    grid_size: &GridSize,
) -> u32 {
    let mut total_moves = 0;

    // Round 1: move east-bound sea cucumbers.
    let snapshot = sea_cucumbers.clone();
    for row in 0..grid_size.0 {
        for col in 0..grid_size.1 {
            if let Some(SeaCucumber::East) = snapshot.get(&(row, col)) {
                let next_pos = if col == grid_size.1 - 1 {
                    (row, 0)
                } else {
                    (row, col + 1)
                };

                if !snapshot.contains_key(&next_pos) {
                    sea_cucumbers.entry(next_pos).or_insert(SeaCucumber::East);
                    sea_cucumbers.remove(&(row, col));

                    total_moves += 1;
                }
            }
        }
    }

    // Round 2: move south-bound sea cucumbers.
    let snapshot = sea_cucumbers.clone();
    for row in 0..grid_size.0 {
        for col in 0..grid_size.1 {
            if let Some(SeaCucumber::South) = snapshot.get(&(row, col)) {
                let next_pos = if row == grid_size.0 - 1 {
                    (0, col)
                } else {
                    (row + 1, col)
                };

                if !snapshot.contains_key(&next_pos) {
                    sea_cucumbers.entry(next_pos).or_insert(SeaCucumber::South);
                    sea_cucumbers.remove(&(row, col));

                    total_moves += 1;
                }
            }
        }
    }

    total_moves
}

#[cfg(test)]
mod tests {
    use aoc_cli::trim_input;

    use super::*;

    const EXAMPLE_INPUT: &str = r"
v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(trim_input(EXAMPLE_INPUT))?, 58);

        Ok(())
    }
}
