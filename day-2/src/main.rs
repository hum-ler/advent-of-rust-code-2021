use anyhow::{anyhow, Result};

use aoc_cli::{get_part, Part};

fn main() {
    match get_part("inputs/day-2.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: &str) -> Result<u32> {
    let coord = input.lines().try_fold((0, 0), |acc, line| {
        // Assumption: depth does not go below 0.

        let (prev_x, prev_y) = acc;

        match parse_value(line) {
            Ok(x) if line.starts_with("forward") => Ok((prev_x + x, prev_y)),
            Ok(y) if line.starts_with("down") => Ok((prev_x, prev_y + y)),
            Ok(y) if line.starts_with("up") => Ok((
                prev_x,
                prev_y.checked_sub(y).ok_or(anyhow!("depth underflow"))?,
            )),
            Err(error) => Err(anyhow!("Unable to parse value: {}", error)),
            Ok(_) => Err(anyhow!("Unhandled line: {}", line)),
        }
    })?;

    Ok(coord.0 * coord.1)
}

fn part_2(input: &str) -> Result<u32> {
    let coord = input.lines().try_fold((0, 0, 0), |acc, line| {
        // Assumption: aim does not go below 0.

        let (prev_x, prev_y, prev_aim) = acc;

        match parse_value(line) {
            Ok(x) if line.starts_with("forward") => {
                Ok((prev_x + x, prev_y + x * prev_aim, prev_aim))
            }
            Ok(aim) if line.starts_with("down") => Ok((prev_x, prev_y, prev_aim + aim)),
            Ok(aim) if line.starts_with("up") => Ok((
                prev_x,
                prev_y,
                prev_aim.checked_sub(aim).ok_or(anyhow!("aim underflow"))?,
            )),
            Err(error) => Err(anyhow!("Unable to parse value: {}", error)),
            Ok(_) => Err(anyhow!("Unhandled line: {}", line)),
        }
    })?;

    Ok(coord.0 * coord.1)
}

fn parse_value(line: &str) -> Result<u32> {
    let Some((_, value)) = line.split_once(" ") else {
        return Err(anyhow!("Cannot split line: {}", line));
    };

    Ok(value.parse()?)
}

#[cfg(test)]
mod tests {
    use aoc_cli::trim_input;

    use super::*;

    const EXAMPLE_INPUT: &str = r"
forward 5
down 5
forward 8
up 3
down 8
forward 2
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(trim_input(EXAMPLE_INPUT))?, 150);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(trim_input(EXAMPLE_INPUT))?, 900);

        Ok(())
    }
}
