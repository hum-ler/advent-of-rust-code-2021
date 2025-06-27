use anyhow::{Result, anyhow};

use cli::{Part, get_part};

fn main() {
    match get_part("inputs/day-11.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{error:?}"),
    }
}

fn part_1(input: &str) -> Result<u32> {
    let mut grid = parse_input_into_grid(input)?;

    let mut flashes = 0;

    for _ in 0..100 {
        grid = step(grid, &mut flashes);
    }

    Ok(flashes)
}

fn part_2(input: &str) -> Result<usize> {
    let mut grid = parse_input_into_grid(input)?;

    let mut steps = 1;

    loop {
        let mut flashes = 0;
        grid = step(grid, &mut flashes);
        if flashes == 100 {
            break;
        }

        steps += 1;
    }

    Ok(steps)
}

type Coord = (usize, usize);

type Grid = [[u8; 10]; 10];

fn parse_input_into_grid(input: &str) -> Result<Grid> {
    let mut grid = [[0; 10]; 10];

    let lines = input.lines().collect::<Vec<_>>();
    if lines.len() != 10 {
        return Err(anyhow!("Incorrect input size: {}", input));
    }

    for (row, line) in lines.into_iter().enumerate() {
        if line.len() != 10 {
            return Err(anyhow!("Incorrect line length: {}", line));
        }

        for (col, byte) in line.bytes().enumerate() {
            grid[row][col] = byte - b'0';
        }
    }

    Ok(grid)
}

fn step(mut grid: Grid, flashes: &mut u32) -> Grid {
    // Add 1 throughout.
    add_one(&mut grid);

    // Resolve explosions.
    let mut pending_flashes = check_pending_flashes(&grid);
    while !pending_flashes.is_empty() {
        for pending_flash in pending_flashes {
            flash(pending_flash, &mut grid);
        }

        pending_flashes = check_pending_flashes(&grid);
    }

    // Substitute explosions with 0 and update flashes.
    *flashes += reset_flashes(&mut grid);

    grid
}

fn add_one(grid: &mut Grid) {
    grid.iter_mut()
        .for_each(|row| row.iter_mut().for_each(|byte| *byte += 1));
}

/// Looks for values that are greater than 9 that are not already flashed (u8::MAX).
fn check_pending_flashes(grid: &Grid) -> Vec<Coord> {
    let mut coords = Vec::new();

    for (row, bytes) in grid.iter().enumerate() {
        for (col, byte) in bytes.iter().enumerate() {
            if *byte > 9 && *byte != u8::MAX {
                coords.push((row, col));
            }
        }
    }

    coords
}

/// Marks pos as flashed (u8::MAX) and adds 1 to all neighbours.
fn flash(pos: Coord, grid: &mut Grid) {
    grid[pos.0][pos.1] = u8::MAX;

    if pos.0 > 0 {
        grid[pos.0 - 1][pos.1] = grid[pos.0 - 1][pos.1].saturating_add(1);
    }
    if pos.0 > 0 && pos.1 < 9 {
        grid[pos.0 - 1][pos.1 + 1] = grid[pos.0 - 1][pos.1 + 1].saturating_add(1);
    }
    if pos.1 < 9 {
        grid[pos.0][pos.1 + 1] = grid[pos.0][pos.1 + 1].saturating_add(1);
    }
    if pos.0 < 9 && pos.1 < 9 {
        grid[pos.0 + 1][pos.1 + 1] = grid[pos.0 + 1][pos.1 + 1].saturating_add(1);
    }
    if pos.0 < 9 {
        grid[pos.0 + 1][pos.1] = grid[pos.0 + 1][pos.1].saturating_add(1);
    }
    if pos.0 < 9 && pos.1 > 0 {
        grid[pos.0 + 1][pos.1 - 1] = grid[pos.0 + 1][pos.1 - 1].saturating_add(1);
    }
    if pos.1 > 0 {
        grid[pos.0][pos.1 - 1] = grid[pos.0][pos.1 - 1].saturating_add(1);
    }
    if pos.0 > 0 && pos.1 > 0 {
        grid[pos.0 - 1][pos.1 - 1] = grid[pos.0 - 1][pos.1 - 1].saturating_add(1);
    }
}

/// Resets flashes (u8::MAX) to 0.
fn reset_flashes(grid: &mut Grid) -> u32 {
    let mut flashes = 0;

    grid.iter_mut().for_each(|row| {
        row.iter_mut().for_each(|byte| {
            if *byte == u8::MAX {
                *byte = 0;
                flashes += 1;
            }
        })
    });

    flashes
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    const EXAMPLE_INPUT: &str = r"
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(trim_newlines(EXAMPLE_INPUT))?, 1656);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(trim_newlines(EXAMPLE_INPUT))?, 195);

        Ok(())
    }
}
