use anyhow::Result;

use aoc_cli::{get_part, Part};

fn main() {
    match get_part("inputs/day-1.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: &str) -> Result<usize> {
    let depths = input
        .lines()
        .map(str::parse::<usize>)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(depths
        .windows(2)
        .fold(0, |acc, pair| if pair[0] < pair[1] { acc + 1 } else { acc }))
}

fn part_2(input: &str) -> Result<usize> {
    let depths = input
        .lines()
        .map(str::parse::<usize>)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(depths
        .windows(3)
        .fold((0, None::<usize>), |acc, triplet| {
            let sum = triplet.iter().sum();

            match acc {
                (_, None) => (0, Some(sum)),
                (count, Some(prev_sum)) if sum > prev_sum => (count + 1, Some(sum)),
                (count, _) => (count, Some(sum)),
            }
        })
        .0)
}

#[cfg(test)]
mod tests {
    use aoc_cli::trim_input;

    use super::*;

    const EXAMPLE_INPUT: &str = r"
199
200
208
210
200
207
240
269
260
263
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(trim_input(EXAMPLE_INPUT))?, 7);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(trim_input(EXAMPLE_INPUT))?, 5);

        Ok(())
    }
}
