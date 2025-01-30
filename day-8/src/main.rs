use std::{
    collections::HashMap,
    ops::{Add, Deref, Sub},
    str::FromStr,
};

use anyhow::{anyhow, Result};

use aoc_cli::{get_part, Part};

fn main() {
    match get_part("inputs/day-8.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .map(|line| {
            let Some((_, output_part)) = line.split_once(" | ") else {
                return Err(anyhow!(
                    "Cannot split line into signal and output: {}",
                    line
                ));
            };

            Ok(output_part)
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .map(|output| {
            output
                .split_ascii_whitespace()
                .filter(|segments| matches!(segments.len(), 2 | 3 | 4 | 7))
                .count()
        })
        .sum())
}

fn part_2(input: &str) -> Result<u32> {
    split_input_into_signal_and_output(input)?
        .into_iter()
        .map(|(signal, output)| {
            let digits = deduce_digits(signal)?;

            Ok(interpret_output(output, &digits))
        })
        .sum()
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct SegmentedDisplay {
    segments: Vec<u8>,
}

impl FromStr for SegmentedDisplay {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut segments = s.as_bytes().to_vec();
        segments.sort();

        Ok(Self { segments })
    }
}

impl Deref for SegmentedDisplay {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.segments.as_slice()
    }
}

impl Sub<&SegmentedDisplay> for SegmentedDisplay {
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self::Output {
        Self {
            segments: self
                .iter()
                .filter(|&segment| !rhs.contains(segment))
                .copied()
                .collect(),
        }
    }
}

impl Add<&SegmentedDisplay> for SegmentedDisplay {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        let mut segments = self.segments;
        segments.extend(rhs.iter());
        segments.dedup();
        segments.sort();

        Self { segments }
    }
}

impl SegmentedDisplay {
    fn intersect(self, rhs: &Self) -> Self {
        Self {
            segments: self
                .iter()
                .filter(|&segment| rhs.contains(segment))
                .copied()
                .collect(),
        }
    }
}

fn split_input_into_signal_and_output(
    input: &str,
) -> Result<Vec<(Vec<SegmentedDisplay>, Vec<SegmentedDisplay>)>> {
    input
        .lines()
        .map(|line| {
            let Some((signal_part, output_part)) = line.split_once(" | ") else {
                return Err(anyhow!(
                    "Cannot split line into signal and output: {}",
                    line
                ));
            };

            Ok((signal_part, output_part))
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .map(|(signal_part, output_part)| {
            let signal = signal_part
                .split_ascii_whitespace()
                .map(SegmentedDisplay::from_str)
                .collect::<Result<Vec<_>>>()?;
            let output = output_part
                .split_ascii_whitespace()
                .map(SegmentedDisplay::from_str)
                .collect::<Result<Vec<_>>>()?;

            Ok((signal, output))
        })
        .collect::<Result<Vec<_>>>()
}

/// Figures out which digit each SegmentDisplay in signal represent.
fn deduce_digits(signal: Vec<SegmentedDisplay>) -> Result<HashMap<SegmentedDisplay, u32>> {
    // Segment count:
    // 8 => 7
    // 0, 6, 9 => 6
    // 2, 3, 5 => 5
    // 4 => 4
    // 7 => 3
    // 1 => 2

    let mut digits = HashMap::new();

    // 8, 4, 7, 1 are identified.
    let Some(digit_8) = signal.iter().find(|digit| digit.len() == 7) else {
        return Err(anyhow!("Cannot deduce digit 8"));
    };
    digits.entry(digit_8.clone()).or_insert(8);

    let Some(digit_4) = signal.iter().find(|digit| digit.len() == 4) else {
        return Err(anyhow!("Cannot deduce digit 4"));
    };
    digits.entry(digit_4.clone()).or_insert(4);

    let Some(digit_7) = signal.iter().find(|digit| digit.len() == 3) else {
        return Err(anyhow!("Cannot deduce digit 7"));
    };
    digits.entry(digit_7.clone()).or_insert(7);

    let Some(digit_1) = signal.iter().find(|digit| digit.len() == 2) else {
        return Err(anyhow!("Cannot deduce digit 1"));
    };
    digits.entry(digit_1.clone()).or_insert(1);

    let six_segments = signal
        .iter()
        .filter(|digit| digit.len() == 6)
        .collect::<Vec<_>>();
    let five_segments = signal
        .iter()
        .filter(|digit| digit.len() == 5)
        .collect::<Vec<_>>();

    // 2 ^ 3 ^ 5 = triple-horizontal.
    // 7 - 1 = top-horizontal.
    // triple-horizontal - 7 - 4 = bottom-horizontal.
    // triple-horizontal - top-horizontal - bottom-horizontal = middle-horizontal.
    let triple_horizontal = five_segments
        .iter()
        .fold(digit_8.clone(), |acc, digit| acc.intersect(digit));
    let top_horizontal = digit_7.clone() - digit_4;
    let bottom_horizontal = triple_horizontal.clone() - digit_7 - digit_4;
    let middle_horizontal = triple_horizontal.clone() - &top_horizontal - &bottom_horizontal;

    // middle-horizontal => 0 is identified.
    let Some(digit_0) = six_segments
        .clone()
        .into_iter()
        .find(|&digit| digit.clone().intersect(&middle_horizontal) != middle_horizontal)
    else {
        return Err(anyhow!("Cannot deduce digit 0"));
    };
    digits.entry(digit_0.clone()).or_insert(0);

    // 1 => 9, 6 are identified.
    let Some(digit_9) = six_segments.clone().into_iter().find(|&digit| {
        if digit == digit_0 {
            return false;
        }

        digit.clone().intersect(digit_1) == *digit_1
    }) else {
        return Err(anyhow!("Cannot deduce digit 9"));
    };
    digits.entry(digit_9.clone()).or_insert(9);

    let Some(digit_6) = six_segments
        .clone()
        .into_iter()
        .find(|&digit| digit != digit_0 && digit != digit_9)
    else {
        return Err(anyhow!("Cannot deduce digit 6"));
    };
    digits.entry(digit_6.clone()).or_insert(6);

    // 8 - 6 = top-right-vertical.
    // 1 - top-right-vertical = bottom-right-vertical.
    let top_right_vertical = digit_8.clone() - digit_6;
    let bottom_right_vertical = digit_1.clone() - &top_right_vertical;

    // 2, 3, 5 are identified.
    let Some(digit_3) = five_segments.clone().into_iter().find(|&digit| {
        digit.clone().intersect(&top_right_vertical) == top_right_vertical
            && digit.clone().intersect(&bottom_right_vertical) == bottom_right_vertical
    }) else {
        return Err(anyhow!("Cannot deduce digit 3"));
    };
    digits.entry(digit_3.clone()).or_insert(3);

    let Some(digit_2) = five_segments.clone().into_iter().find(|&digit| {
        digit.clone().intersect(&top_right_vertical) == top_right_vertical
            && digit.clone().intersect(&bottom_right_vertical) != bottom_right_vertical
    }) else {
        return Err(anyhow!("Cannot deduce digit 2"));
    };
    digits.entry(digit_2.clone()).or_insert(2);

    let Some(digit_5) = five_segments.clone().into_iter().find(|&digit| {
        digit.clone().intersect(&top_right_vertical) != top_right_vertical
            && digit.clone().intersect(&bottom_right_vertical) == bottom_right_vertical
    }) else {
        return Err(anyhow!("Cannot deduce digit 5"));
    };
    digits.entry(digit_5.clone()).or_insert(5);

    Ok(digits)
}

/// Converts output to the corresponding number using the given deduced digits.
fn interpret_output(output: Vec<SegmentedDisplay>, digits: &HashMap<SegmentedDisplay, u32>) -> u32 {
    output
        .into_iter()
        .fold(0u32, |acc, value| acc * 10 + digits[&value])
}

#[cfg(test)]
mod tests {
    use aoc_cli::trim_input;

    use super::*;

    const EXAMPLE_INPUT: &str = r"
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(trim_input(EXAMPLE_INPUT))?, 26);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(trim_input(EXAMPLE_INPUT))?, 61229);

        Ok(())
    }
}
