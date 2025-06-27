use std::num::ParseIntError;

use anyhow::{Result, anyhow};

use cli::{Part, get_part};

fn main() {
    match get_part("inputs/day-4.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{error:?}"),
    }
}

fn part_1(input: &str) -> Result<u32> {
    let Some((sequence_part, bingo_cards_part)) = input.split_once("\n\n") else {
        return Err(anyhow!(
            "Cannot divide input into sequence and bingo cards: {}",
            input
        ));
    };

    let sequence = parse_sequence(sequence_part)?;

    let mut bingo_cards = bingo_cards_part
        .split_terminator("\n\n")
        .map(parse_bingo_card)
        .collect::<Result<Vec<_>, _>>()?;

    for number in sequence {
        for bingo_card in &mut bingo_cards {
            if bingo(number, bingo_card)? {
                return Ok(bingo_card_value(bingo_card) * number);
            }
        }
    }

    Err(anyhow!("Cannot reach bingo"))
}

fn part_2(input: &str) -> Result<u32> {
    let Some((sequence_part, bingo_cards_part)) = input.split_once("\n\n") else {
        return Err(anyhow!(
            "Cannot divide input into sequence and bingo cards: {}",
            input
        ));
    };

    let sequence = parse_sequence(sequence_part)?;

    let mut bingo_cards = bingo_cards_part
        .split_terminator("\n\n")
        .map(parse_bingo_card)
        .collect::<Result<Vec<_>, _>>()?;

    for number in sequence {
        let len = bingo_cards.len();

        // Reversing the index in case multiple cards need to be removed.
        for index in (0..len).rev() {
            let cards_left = bingo_cards.len(); // in case other cards are removed
            let bingo_card = &mut bingo_cards[index];

            if bingo(number, bingo_card)? {
                if cards_left == 1 {
                    return Ok(bingo_card_value(bingo_card) * number);
                }

                bingo_cards.remove(index);
            }
        }
    }

    Err(anyhow!("Cannot reach bingo"))
}

fn parse_sequence(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input.split_terminator(",").map(str::parse).collect()
}

fn parse_bingo_card(input: &str) -> Result<Vec<Vec<Option<u32>>>, ParseIntError> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(str::parse)
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .map(Option::Some)
                .map(Result::Ok)
                .collect()
        })
        .collect::<Result<Vec<_>, _>>()
}

/// Checks a card for number match and bingo.
fn bingo(number: u32, bingo_card: &mut [Vec<Option<u32>>]) -> Result<bool> {
    // Check for match.
    if let Some(row) = bingo_card
        .iter()
        .position(|row| row.contains(&Some(number)))
    {
        let Some(col) = bingo_card[row]
            .iter()
            .position(|value| value == &Some(number))
        else {
            return Err(anyhow!("Cannot retrieve number position"));
        };

        // Mark it.
        bingo_card[row][col] = None;

        // Check row for bingo.
        if bingo_card[row].iter().all(|value| value.is_none()) {
            return Ok(true);
        }

        // Check col for bingo.
        if bingo_card.iter().all(|row| row[col].is_none()) {
            return Ok(true);
        }
    }

    Ok(false)
}

/// Sums up the remaining numbers in bingo_card.
fn bingo_card_value(bingo_card: &[Vec<Option<u32>>]) -> u32 {
    bingo_card
        .iter()
        .flat_map(|row| row.iter().map(|value| value.unwrap_or_default()))
        .sum()
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    const EXAMPLE_INPUT: &str = r"
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(trim_newlines(EXAMPLE_INPUT))?, 4512);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(trim_newlines(EXAMPLE_INPUT))?, 1924);

        Ok(())
    }
}
