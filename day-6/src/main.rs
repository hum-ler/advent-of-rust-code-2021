use std::collections::HashMap;

use anyhow::Result;

use aoc_cli::{get_part, Part};

fn main() {
    match get_part("inputs/day-6.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: &str) -> Result<usize> {
    let mut fishes = input
        .split_terminator(",")
        .map(str::parse::<u8>)
        .collect::<Result<Vec<_>, _>>()?;

    for _ in 0..80 {
        let fish_to_add = fishes.iter().filter(|&fish| *fish == 0).count();

        fishes = fishes
            .into_iter()
            .map(|fish| if fish == 0 { 6 } else { fish - 1 })
            .collect();

        fishes.extend(vec![8; fish_to_add]);
    }

    Ok(fishes.len())
}

fn part_2(input: &str) -> Result<usize> {
    let fishes = input
        .split_terminator(",")
        .map(str::parse::<u8>)
        .collect::<Result<Vec<_>, _>>()?;

    let mut cache = HashMap::new();
    let fish_counts = (1..=5) // fish runs from 1 to 5 inclusive
        .map(|fish_timer| self_plus_progeny(fish_timer, 256, &mut cache))
        .collect::<Vec<_>>();

    Ok(fishes
        .into_iter()
        .map(|fish| fish_counts[fish as usize - 1])
        .sum())
}

fn self_plus_progeny(
    fish_timer: u8,
    days_left: u16,
    cache: &mut HashMap<(u8, u16), usize>,
) -> usize {
    if cache.contains_key(&(fish_timer, days_left)) {
        return cache[&(fish_timer, days_left)];
    }

    if fish_timer as u16 >= days_left {
        return 1;
    }

    let count = if fish_timer == 0 {
        self_plus_progeny(6, days_left - 1, cache) + self_plus_progeny(8, days_left - 1, cache)
    } else {
        self_plus_progeny(fish_timer - 1, days_left - 1, cache)
    };

    *cache.entry((fish_timer, days_left)).or_insert(count)
}

#[cfg(test)]
mod tests {
    use aoc_cli::trim_input;

    use super::*;

    const EXAMPLE_INPUT: &str = "3,4,3,1,2";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(trim_input(EXAMPLE_INPUT))?, 5934);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(trim_input(EXAMPLE_INPUT))?, 26984457539);

        Ok(())
    }
}
