use std::cmp::max;

use anyhow::{anyhow, Result};

use aoc_cli::{get_part, Part};

fn main() {
    match get_part("inputs/day-18.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: &str) -> Result<u64> {
    let pairs = input
        .lines()
        .map(parse_input_into_pair)
        .collect::<Result<Vec<_>>>()?;

    let sum = pairs.into_iter().try_fold(Vec::new(), |acc, pair| {
        if acc.is_empty() {
            Ok(pair)
        } else {
            add(acc, pair)
        }
    })?;

    magnitude(&sum)
}

fn part_2(input: &str) -> Result<u64> {
    let pairs = input
        .lines()
        .map(parse_input_into_pair)
        .collect::<Result<Vec<_>>>()?;

    let mut max_magnitude = 0;
    for first_pair in &pairs {
        for second_pair in &pairs {
            if first_pair == second_pair {
                continue;
            }

            let magnitude = magnitude(&add(first_pair.clone(), second_pair.clone())?)?;

            max_magnitude = max(max_magnitude, magnitude);
        }
    }

    Ok(max_magnitude)
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Symbol {
    Start,
    End,
    Separator,
    Number(u8),
}

impl Symbol {
    /// Adds a [Symbol::Number] to another [Symbol::Number].
    fn try_add(self, other: Self) -> Result<Self> {
        let Symbol::Number(number) = self else {
            return Err(anyhow!("Attempted to add non-Number Symbol"));
        };
        let Symbol::Number(other_number) = other else {
            return Err(anyhow!("Attempted to add non-Number Symbol"));
        };

        Ok(Self::Number(number + other_number))
    }
}

fn parse_input_into_pair(input: &str) -> Result<Vec<Symbol>> {
    let mut symbols = Vec::new();

    input.bytes().try_for_each(|byte| {
        match byte {
            b'[' => symbols.push(Symbol::Start),
            b']' => symbols.push(Symbol::End),
            b',' => symbols.push(Symbol::Separator),
            d if d.is_ascii_digit() => symbols.push(Symbol::Number(d - b'0')),
            x => return Err(anyhow!("Invalid symbol: {}", x)),
        }

        Ok(())
    })?;

    Ok(symbols)
}

/// Adds a pair to another pair.
fn add(pair: Vec<Symbol>, other: Vec<Symbol>) -> Result<Vec<Symbol>> {
    let mut new_pair = vec![Symbol::Start];

    new_pair.extend(pair);

    new_pair.push(Symbol::Separator);

    new_pair.extend(other);

    new_pair.push(Symbol::End);

    reduce(new_pair)
}

/// Reduces a pair repeatably, until no more explosion or splitting is necessary.
fn reduce(mut pair: Vec<Symbol>) -> Result<Vec<Symbol>> {
    loop {
        if explode(&mut pair)? {
            continue;
        }

        if split(&mut pair)? {
            continue;
        }

        break;
    }

    Ok(pair)
}

/// Explodes the first occurence of a [Symbol::Number] pair above nest depth of 4.
fn explode(pair: &mut Vec<Symbol>) -> Result<bool> {
    // Find an exploding pair.
    let (mut explode_start, mut explode_end) = (0, 0);
    let mut depth = 0;
    for (index, symbol) in pair.iter().enumerate() {
        match symbol {
            Symbol::Start => depth += 1,
            Symbol::End => {
                if depth > 4 {
                    explode_start = index - 4;
                    explode_end = index;

                    break;
                } else {
                    depth -= 1;
                }
            }
            _ => (),
        }
    }

    if explode_end > 0 {
        // Found an exploding pair.

        // Add left number to the left.
        let number_to_the_left = pair[0..explode_start]
            .iter()
            .rposition(|symbol| matches!(symbol, Symbol::Number(_)));
        if let Some(number_to_the_left) = number_to_the_left {
            pair[number_to_the_left] = pair[number_to_the_left].try_add(pair[explode_start + 1])?;
        }

        // Add right number to the right.
        let number_to_the_right = pair[explode_end + 1..]
            .iter()
            .position(|symbol| matches!(symbol, Symbol::Number(_)));
        if let Some(number_to_the_right) = number_to_the_right {
            pair[explode_end + 1 + number_to_the_right] =
                pair[explode_end + 1 + number_to_the_right].try_add(pair[explode_end - 1])?;
        }

        // Replace the exploding pair with 0.
        pair.remove(explode_end);
        pair.remove(explode_end - 1);
        pair.remove(explode_end - 2);
        pair.remove(explode_end - 3);
        pair[explode_start] = Symbol::Number(0);

        Ok(true)
    } else {
        Ok(false)
    }
}

/// Splits the first occurence of a [Symbol::Number] above 9.
fn split(pair: &mut Vec<Symbol>) -> Result<bool> {
    // Find a splitting number.
    let to_split = pair.iter().position(|symbol| {
        if let Symbol::Number(number) = symbol {
            if *number > 9 {
                return true;
            }
        }

        false
    });

    // Replace splitting number with pair.
    if let Some(to_split) = to_split {
        let Symbol::Number(number) = pair[to_split] else {
            return Err(anyhow!("Cannot cast Symbol to Number"));
        };

        pair[to_split] = Symbol::End;
        pair.insert(to_split, Symbol::Number(number.div_ceil(2)));
        pair.insert(to_split, Symbol::Separator);
        pair.insert(to_split, Symbol::Number(number / 2));
        pair.insert(to_split, Symbol::Start);

        Ok(true)
    } else {
        Ok(false)
    }
}

/// Calculates the magnitude of a pair.
fn magnitude(pair: &[Symbol]) -> Result<u64> {
    let separator_index = find_separator(pair)?;

    let left = if separator_index == 2 {
        let Symbol::Number(number) = pair[1] else {
            return Err(anyhow!(
                "Invalid pair (expected Number at pos 2): {:?}",
                pair
            ));
        };

        number as u64
    } else {
        magnitude(&pair[1..separator_index])?
    };

    let right = if pair.len() - separator_index == 3 {
        let Symbol::Number(number) = pair[separator_index + 1] else {
            return Err(anyhow!(
                "Invalid pair (expected Number at pos {}): {:?}",
                separator_index + 1,
                pair,
            ));
        };

        number as u64
    } else {
        magnitude(&pair[separator_index + 1..pair.len() - 1])?
    };

    Ok(3 * left + 2 * right)
}

/// Finds the pos of the [Symbol::Separator] that divides the pair into left and right halves.
fn find_separator(pair: &[Symbol]) -> Result<usize> {
    let mut depth = 0;
    for index in 1..pair.len() - 1 {
        match pair[index] {
            Symbol::Start => depth += 1,
            Symbol::End => {
                if depth == 0 {
                    return Err(anyhow!(
                        "Invalid pair (unexpected closing bracket at index {}): {:?}",
                        index,
                        pair
                    ));
                }

                depth -= 1;
            }
            Symbol::Number(_) => (),
            Symbol::Separator => {
                if depth == 0 {
                    return Ok(index);
                }
            }
        }
    }

    Err(anyhow!("Cannot find separator at depth 0"))
}

#[cfg(test)]
mod tests {
    use aoc_cli::trim_input;

    use super::*;

    const EXAMPLE_INPUT: &str = r"
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(trim_input(EXAMPLE_INPUT))?, 4140);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(trim_input(EXAMPLE_INPUT))?, 3993);

        Ok(())
    }
}
