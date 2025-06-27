use std::{
    cmp::{max, min},
    collections::HashMap,
};

use anyhow::{Result, anyhow};

use cli::{Part, get_part};

fn main() {
    match get_part("inputs/day-21.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{error:?}"),
    }
}

fn part_1(input: &str) -> Result<u64> {
    let mut player_pos = parse_input_into_starting_positions(input)?;
    let mut player_scores = (0, 0);
    let mut die_roll_count = 0;

    while player_scores.0 < 1000 && player_scores.1 < 1000 {
        let player = (die_roll_count as usize / 3) % 2;

        if player == 0 {
            player_pos.0 = wrap_number(
                player_pos.0
                    + roll_100_sided_die(&mut die_roll_count)
                    + roll_100_sided_die(&mut die_roll_count)
                    + roll_100_sided_die(&mut die_roll_count),
                10,
            );
            player_scores.0 += player_pos.0;
        } else {
            player_pos.1 = wrap_number(
                player_pos.1
                    + roll_100_sided_die(&mut die_roll_count)
                    + roll_100_sided_die(&mut die_roll_count)
                    + roll_100_sided_die(&mut die_roll_count),
                10,
            );

            player_scores.1 += player_pos.1;
        }
    }

    Ok(min(player_scores.0, player_scores.1) * die_roll_count)
}

fn part_2(input: &str) -> Result<u64> {
    let player_pos = parse_input_into_starting_positions(input)?;

    let player_wins = play_game(player_pos);

    Ok(max(player_wins.0, player_wins.1))
}

fn parse_input_into_starting_positions(input: &str) -> Result<(u64, u64)> {
    let starting_pos = input
        .lines()
        .map(|line| {
            let Some((_, pos)) = line.split_once(": ") else {
                return Err(anyhow!("Cannot split line to get pos: {}", line));
            };

            pos.parse()
                .map_err(|error| anyhow!("Cannot parse pos {}: {}", pos, error))
        })
        .collect::<Result<Vec<_>>>()?;

    if starting_pos.len() != 2 {
        return Err(anyhow!("Invalid input len: {}", input));
    }

    Ok((starting_pos[0], starting_pos[1]))
}

fn roll_100_sided_die(die_roll_count: &mut u64) -> u64 {
    *die_roll_count += 1;

    wrap_number(*die_roll_count, 100)
}

/// Wrap a number to the range 1..=modulus.
fn wrap_number(number: u64, modulus: u64) -> u64 {
    let remainder = number % modulus;

    if remainder == 0 { modulus } else { remainder }
}

#[derive(Eq, Hash, PartialEq)]
struct CacheKey {
    player: u64,
    dice_roll: u64,
    player_pos: (u64, u64),
    player_score: (u64, u64),
}

type Cache = HashMap<CacheKey, (u64, u64)>;

/// Counts the number of wins for both players.
///
/// Returns a tuple of (player_0_wins, player_1_wins).
fn count_wins(
    player: u64,
    dice_roll: u64,
    mut player_pos: (u64, u64),
    mut player_score: (u64, u64),
    cache: &mut Cache,
) -> (u64, u64) {
    let cache_key = CacheKey {
        player,
        dice_roll,
        player_pos,
        player_score,
    };

    if cache.contains_key(&cache_key) {
        return cache[&cache_key];
    }

    if player == 0 {
        player_pos.0 = wrap_number(player_pos.0 + dice_roll, 10);
        player_score.0 += player_pos.0;
        if player_score.0 >= 21 {
            return (1, 0);
        }
    } else {
        player_pos.1 = wrap_number(player_pos.1 + dice_roll, 10);
        player_score.1 += player_pos.1;
        if player_score.1 >= 21 {
            return (0, 1);
        }
    }

    let next_player = (player + 1) % 2;

    let player_wins = distribution_3d3()
        .into_iter()
        .map(|(dice_roll, chances)| {
            factor_into_tuple(
                count_wins(next_player, dice_roll, player_pos, player_score, cache),
                chances,
            )
        })
        .reduce(|acc, player_wins| (acc.0 + player_wins.0, acc.1 + player_wins.1))
        .unwrap_or((0, 0));

    *cache.entry(cache_key).or_insert(player_wins)
}

/// Multiples the values in the given tuple by factor.
fn factor_into_tuple(tuple: (u64, u64), factor: u64) -> (u64, u64) {
    (tuple.0 * factor, tuple.1 * factor)
}

/// The list of (sum_of_dice_roll, number_of_combinations) for 3 rolls of 3-sided die.
fn distribution_3d3() -> [(u64, u64); 7] {
    [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)]
}

/// Plays a game with the given starting positions.
///
/// Returns a tuple of (player_0_wins, player_1_wins).
fn play_game(player_pos: (u64, u64)) -> (u64, u64) {
    let mut cache = HashMap::new();

    distribution_3d3()
        .into_iter()
        .map(|(dice_roll, chances)| {
            factor_into_tuple(
                count_wins(0, dice_roll, player_pos, (0, 0), &mut cache),
                chances,
            )
        })
        .reduce(|acc, wins| (acc.0 + wins.0, acc.1 + wins.1))
        .unwrap_or((0, 0))
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    const EXAMPLE_INPUT: &str = r"
Player 1 starting position: 4
Player 2 starting position: 8
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(trim_newlines(EXAMPLE_INPUT))?, 739785);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(trim_newlines(EXAMPLE_INPUT))?, 444356092776315);

        Ok(())
    }
}
