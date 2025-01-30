use std::{collections::HashMap, str::FromStr};

use anyhow::{anyhow, Result};

use aoc_cli::{get_part, Part};

fn main() {
    match get_part("inputs/day-5.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: &str) -> Result<usize> {
    // Map of Coords against LineSegments crossing it.
    let mut coords: HashMap<Coord, Vec<LineSegment>> = HashMap::new();

    input
        .lines()
        .map(LineSegment::from_str)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .filter(LineSegment::is_vertical_or_horizontal)
        .for_each(|line_segment| {
            for coord in line_segment.to_coords() {
                coords.entry(coord).or_default().push(line_segment);
            }
        });

    Ok(coords
        .values()
        .filter(|line_segments| line_segments.len() > 1)
        .count())
}

fn part_2(input: &str) -> Result<usize> {
    let mut coords: HashMap<Coord, Vec<LineSegment>> = HashMap::new();

    input
        .lines()
        .map(LineSegment::from_str)
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .for_each(|line_segment| {
            for coord in line_segment.to_coords() {
                coords.entry(coord).or_default().push(line_segment);
            }
        });

    Ok(coords
        .values()
        .filter(|line_segments| line_segments.len() > 1)
        .count())
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Coord {
    x: u32,
    y: u32,
}

impl FromStr for Coord {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let Some((x, y)) = s.split_once(",") else {
            return Err(anyhow!("Cannot split s: {}", s));
        };

        let x = x.parse()?;
        let y = y.parse()?;

        Ok(Coord { x, y })
    }
}

#[derive(Clone, Copy)]
struct LineSegment {
    start: Coord,
    end: Coord,
}

impl FromStr for LineSegment {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let Some((start, end)) = s.split_once(" -> ") else {
            return Err(anyhow!("Cannot split s: {}", s));
        };

        let start = Coord::from_str(start)?;
        let end = Coord::from_str(end)?;

        Ok(LineSegment { start, end })
    }
}

impl LineSegment {
    fn is_vertical_or_horizontal(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }

    fn to_coords(self) -> Vec<Coord> {
        // Assumption: self.start != self.end.
        // Assumption: Line must be vertical, horizontal or perfectly 45-degree diagonal.

        let mut coords = Vec::new();

        let x_range: Box<dyn Iterator<Item = _>> = match (self.start.x, self.end.x) {
            (start, end) if start == end => Box::new((start..=start).cycle()), // vertical line
            (start, end) if start < end => Box::new(start..=end),
            (start, end) => Box::new((end..=start).rev()),
        };
        let y_range: Box<dyn Iterator<Item = _>> = match (self.start.y, self.end.y) {
            (start, end) if start == end => Box::new((start..=start).cycle()), // horizontal line
            (start, end) if start < end => Box::new(start..=end),
            (start, end) => Box::new((end..=start).rev()),
        };

        for (x, y) in x_range.zip(y_range) {
            coords.push(Coord { x, y });
        }

        coords
    }
}

#[cfg(test)]
mod tests {
    use aoc_cli::trim_input;

    use super::*;

    const EXAMPLE_INPUT: &str = r"
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(trim_input(EXAMPLE_INPUT))?, 5);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(trim_input(EXAMPLE_INPUT))?, 12);

        Ok(())
    }
}
