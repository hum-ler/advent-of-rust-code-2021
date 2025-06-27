use anyhow::{Result, anyhow};

use cli::{Part, get_part};

fn main() {
    match get_part("inputs/day-7.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{error:?}"),
    }
}

fn part_1(input: &str) -> Result<u32> {
    let crabs = input
        .split_terminator(",")
        .map(str::parse::<u32>)
        .collect::<Result<Vec<_>, _>>()?;

    let Some(&min_pos) = crabs.iter().min() else {
        return Err(anyhow!("Cannot determine min pos"));
    };
    let Some(&max_pos) = crabs.iter().max() else {
        return Err(anyhow!("Cannot determine max pos"));
    };

    (min_pos..=max_pos)
        .map(|pos| crabs.iter().map(|crab| crab.abs_diff(pos)).sum())
        .min()
        .ok_or(anyhow!("Cannot determine min fuel"))
}

fn part_2(input: &str) -> Result<u32> {
    let crabs = input
        .split_terminator(",")
        .map(str::parse::<u32>)
        .collect::<Result<Vec<_>, _>>()?;

    let Some(&min_pos) = crabs.iter().min() else {
        return Err(anyhow!("Cannot determine min pos"));
    };
    let Some(&max_pos) = crabs.iter().max() else {
        return Err(anyhow!("Cannot determine max pos"));
    };

    (min_pos..=max_pos)
        .map(|pos| {
            crabs
                .iter()
                .map(|crab| {
                    let moves = crab.abs_diff(pos);

                    moves * (moves + 1) / 2 // sum of N series
                })
                .sum()
        })
        .min()
        .ok_or(anyhow!("Cannot determine min fuel"))
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    const EXAMPLE_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(trim_newlines(EXAMPLE_INPUT))?, 37);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(trim_newlines(EXAMPLE_INPUT))?, 168);

        Ok(())
    }
}
