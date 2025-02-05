use std::{
    cmp::{max, min},
    collections::HashSet,
};

use anyhow::{anyhow, Result};

use aoc_cli::{get_part, Part};

fn main() {
    match get_part("inputs/day-13.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: &str) -> Result<usize> {
    let (mut dots, folds) = parse_input_into_dots_and_folds(input)?;

    assert!(!folds.is_empty());

    fold_paper(&folds[0], &mut dots);

    Ok(dots.len())
}

fn part_2(input: &str) -> Result<String> {
    let (mut dots, folds) = parse_input_into_dots_and_folds(input)?;

    for fold in &folds {
        fold_paper(fold, &mut dots);
    }

    Ok(print_paper(&dots))
}

type Coord = (usize, usize);

#[derive(PartialEq)]
enum Axis {
    X,
    Y,
}

type Fold = (Axis, usize);

fn parse_input_into_dots_and_folds(input: &str) -> Result<(HashSet<Coord>, Vec<Fold>)> {
    let Some((dots_part, folds_part)) = input.split_once("\n\n") else {
        return Err(anyhow!("Cannot split input into dots and folds: {}", input));
    };

    let dots = dots_part
        .lines()
        .map(parse_dot)
        .collect::<Result<HashSet<_>>>()?;

    let folds = folds_part
        .lines()
        .map(parse_fold)
        .collect::<Result<Vec<_>>>()?;

    Ok((dots, folds))
}

fn parse_dot(input: &str) -> Result<Coord> {
    let Some((x, y)) = input.split_once(",") else {
        return Err(anyhow!("Cannot split input into x and y: {}", input));
    };

    Ok((x.parse()?, y.parse()?))
}

fn parse_fold(input: &str) -> Result<Fold> {
    let Some((axix_part, index)) = input.split_once("=") else {
        return Err(anyhow!("Cannot split input into axis and index: {}", input));
    };

    let axis = if axix_part.ends_with("y") {
        Axis::Y
    } else {
        Axis::X
    };

    Ok((axis, index.parse()?))
}

fn fold_paper(fold: &Fold, dots: &mut HashSet<Coord>) {
    let (axis, index) = fold;

    for dot in dots.clone() {
        if *axis == Axis::X && dot.0 > *index {
            dots.remove(&dot);
            dots.insert((dot.0 - ((dot.0 - *index) * 2), dot.1));
        } else if *axis == Axis::Y && dot.1 > *index {
            dots.remove(&dot);
            dots.insert((dot.0, dot.1 - ((dot.1 - *index) * 2)));
        }
    }
}

fn print_paper(dots: &HashSet<Coord>) -> String {
    // Get bounding box.
    let ((min_x, min_y), (max_x, max_y)) = dots.iter().fold(
        ((usize::MAX, usize::MAX), (usize::MIN, usize::MIN)),
        |mut acc, dot| {
            acc.0 .0 = min(acc.0 .0, dot.0);
            acc.0 .1 = min(acc.0 .1, dot.1);
            acc.1 .0 = max(acc.1 .0, dot.0);
            acc.1 .1 = max(acc.1 .1, dot.1);

            acc
        },
    );

    let mut printout = String::new();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if dots.contains(&(x, y)) {
                printout.push('#');
            } else {
                printout.push(' ');
            }
        }
        printout.push('\n');
    }

    println!("{}", printout);

    printout
}

#[cfg(test)]
mod tests {
    use aoc_cli::trim_input;

    use super::*;

    const EXAMPLE_INPUT: &str = r"
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(trim_input(EXAMPLE_INPUT))?, 17);

        Ok(())
    }
}
