use std::cmp::Ordering;

use anyhow::{Result, anyhow};

use cli::{Part, get_part};

fn main() {
    match get_part("inputs/day-3.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{error:?}"),
    }
}

fn part_1(input: &str) -> Result<u32> {
    let lines = input.lines().collect::<Vec<_>>();

    let var_size = lines.first().map_or(0, |first_line| first_line.len());
    let var_count = lines.len();

    assert!(var_size > 0);
    assert!(var_count > 0);

    let count_ones = lines.into_iter().fold(vec![0; var_size], |mut acc, line| {
        line.bytes().enumerate().for_each(|(col, byte)| {
            if byte == b'1' {
                acc[col] += 1;
            }
        });

        acc
    });

    let Some(gamma) = count_ones
        .into_iter()
        .map(|count| if count > var_count - count { 1 } else { 0 })
        .reduce(|acc, bit| (acc << 1) + bit)
    else {
        return Err(anyhow!("Cannot assemble gamma"));
    };

    let epsilon = u32::from_str_radix(&"1".repeat(var_size), 2)? ^ gamma;

    Ok(gamma * epsilon)
}

fn part_2(input: &str) -> Result<u32> {
    let lines = input.lines().collect::<Vec<_>>();

    let oxygen_generator_rating = u32::from_str_radix(reduce_numbers_to_rating(&lines, 0, 1)?, 2)?;
    let co2_scrubber_rating = u32::from_str_radix(reduce_numbers_to_rating(&lines, 0, 0)?, 2)?;

    Ok(oxygen_generator_rating * co2_scrubber_rating)
}

/// Filters the list input numbers down to a single one.
///
/// Use tie_breaker to determine how the list is filtered:
/// - 0 will favor least common value at bit_to_check, using 0 in case of a tie.
/// - 1 will favor most common value at bit_to_check, using 1 in case of a tie.
fn reduce_numbers_to_rating<'a>(
    numbers: &[&'a str],
    bit_to_check: usize,
    tie_breaker: u8,
) -> Result<&'a str> {
    assert!(tie_breaker == 0 || tie_breaker == 1);
    assert!(numbers.len() > 1);
    assert!(bit_to_check < numbers[0].len());

    let ones_count = numbers
        .iter()
        .filter(|line| line.as_bytes()[bit_to_check] == b'1')
        .count();

    let value_to_match = match (tie_breaker, ones_count.cmp(&(numbers.len() - ones_count))) {
        (0, Ordering::Equal) => b'0',
        (0, Ordering::Greater) => b'0',
        (0, Ordering::Less) => b'1',
        (1, Ordering::Equal) => b'1',
        (1, Ordering::Greater) => b'1',
        (1, Ordering::Less) => b'0',
        (x, _) => return Err(anyhow!("Invalid tie-breaker: {}", x)),
    };

    let filtered_numbers = numbers
        .iter()
        .filter(|number| number.as_bytes()[bit_to_check] == value_to_match)
        .copied()
        .collect::<Vec<_>>();

    if filtered_numbers.len() == 1 {
        return Ok(filtered_numbers[0]);
    }

    if filtered_numbers.is_empty() {
        return Err(anyhow!("All numbers filtered out at bit: {}", bit_to_check));
    }

    reduce_numbers_to_rating(&filtered_numbers, bit_to_check + 1, tie_breaker)
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    const EXAMPLE_INPUT: &str = r"
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(trim_newlines(EXAMPLE_INPUT))?, 198);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(trim_newlines(EXAMPLE_INPUT))?, 230);

        Ok(())
    }
}
