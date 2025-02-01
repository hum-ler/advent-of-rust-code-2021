use std::collections::HashMap;

use anyhow::{anyhow, Result};

use aoc_cli::{get_part, Part};

fn main() {
    match get_part("inputs/day-14.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: &str) -> Result<u32> {
    let (mut polymer, rules) = parse_input_into_template_and_rules(input)?;

    for _ in 0..10 {
        polymer = step(polymer, &rules);
    }

    let mut frequencies = table(&polymer);
    frequencies.sort();

    Ok(frequencies[frequencies.len() - 1] - frequencies[0])
}

fn part_2(input: &str) -> Result<u64> {
    // Re-implement using dynamic programming.

    let (polymer, rules) = parse_input_into_template_and_rules(input)?;

    let table = count_polymer_elements(&polymer, 40, &rules);

    let mut frequencies = table.values().collect::<Vec<_>>();
    frequencies.sort();

    Ok(frequencies[frequencies.len() - 1] - frequencies[0])
}

type RuleMap = HashMap<(u8, u8), u8>;

fn parse_input_into_template_and_rules(input: &str) -> Result<(Vec<u8>, RuleMap)> {
    let Some((template_part, rules_part)) = input.split_once("\n\n") else {
        return Err(anyhow!(
            "Cannot split input into template and rules: {}",
            input
        ));
    };

    let template = template_part.bytes().collect();

    let rules = rules_part
        .lines()
        .map(parse_rule)
        .collect::<Result<HashMap<_, _>>>()?;

    Ok((template, rules))
}

fn parse_rule(input: &str) -> Result<((u8, u8), u8)> {
    let Some((pair_part, insert_part)) = input.split_once(" -> ") else {
        return Err(anyhow!(
            "Cannot split input into pair and insert: {}",
            input
        ));
    };

    assert_eq!(pair_part.len(), 2);
    assert_eq!(insert_part.len(), 1);

    let pair = pair_part.as_bytes();
    let insert = insert_part.as_bytes();

    Ok(((pair[0], pair[1]), insert[0]))
}

// Performs one step.
fn step(polymer: Vec<u8>, rules: &RuleMap) -> Vec<u8> {
    let tail = polymer[polymer.len() - 1];

    let mut polymer = polymer
        .windows(2)
        .flat_map(|pair| {
            if rules.contains_key(&(pair[0], pair[1])) {
                vec![pair[0], rules[&(pair[0], pair[1])]]
            } else {
                vec![pair[0]]
            }
        })
        .collect::<Vec<_>>();

    polymer.push(tail);

    polymer
}

// Tables up the element frequencies.
fn table(polymer: &[u8]) -> Vec<u32> {
    let mut element_counts: HashMap<u8, u32> = HashMap::new();

    polymer
        .iter()
        .for_each(|element| *(element_counts.entry(*element).or_default()) += 1);

    element_counts.values().copied().collect()
}

type FrequencyTable = HashMap<u8, u64>;

type FrequencyTableCache = HashMap<((u8, u8), u8), FrequencyTable>;

fn count_polymer_elements(polymer: &[u8], steps_left: u8, rules: &RuleMap) -> FrequencyTable {
    let mut table = HashMap::new();

    let mut cache = HashMap::new();
    polymer.windows(2).for_each(|pair| {
        let pair_table = count_elements((pair[0], pair[1]), steps_left, rules, &mut cache);

        // Combine everything into table.
        for (k, v) in pair_table {
            *table.entry(k).or_default() += v;
        }
    });

    // Deduct overlaps.
    polymer[1..polymer.len() - 1].iter().for_each(|overlap| {
        table.entry(*overlap).and_modify(|v| *v -= 1);
    });

    table
}

fn count_elements(
    pair: (u8, u8),
    steps_left: u8,
    rules: &RuleMap,
    cache: &mut FrequencyTableCache,
) -> FrequencyTable {
    // Cache hit.
    if cache.contains_key(&(pair, steps_left)) {
        return cache[&(pair, steps_left)].clone();
    }

    // Base case.
    if steps_left == 0 {
        return if pair.0 == pair.1 {
            HashMap::from([(pair.0, 2)])
        } else {
            HashMap::from([(pair.0, 1), (pair.1, 1)])
        };
    }

    let counts = if rules.contains_key(&pair) {
        let insert = rules[&pair];

        let mut left_pair = count_elements((pair.0, insert), steps_left - 1, rules, cache);
        let right_pair = count_elements((insert, pair.1), steps_left - 1, rules, cache);

        // Combine the tables into left_pair.
        for (k, v) in right_pair {
            *left_pair.entry(k).or_default() += v;
        }

        // Deduct the overlap element (insert).
        left_pair.entry(insert).and_modify(|v| *v -= 1);

        left_pair
    } else {
        count_elements(pair, steps_left - 1, rules, cache)
    };

    cache.entry((pair, steps_left)).or_insert(counts).clone()
}

#[cfg(test)]
mod tests {
    use aoc_cli::trim_input;

    use super::*;

    const EXAMPLE_INPUT: &str = r"
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(trim_input(EXAMPLE_INPUT))?, 1588);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(trim_input(EXAMPLE_INPUT))?, 2188189693529);

        Ok(())
    }
}
