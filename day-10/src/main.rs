use anyhow::{Result, anyhow};

use cli::{Part, get_part};

fn main() {
    match get_part("inputs/day-10.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{error:?}"),
    }
}

fn part_1(input: &str) -> Result<u32> {
    input
        .lines()
        .map(|line| {
            if let Some(illegal_close_char) = first_illegal_close_char(line)? {
                score_illegal_close_char(illegal_close_char)
            } else {
                Ok(0)
            }
        })
        .sum()
}

fn part_2(input: &str) -> Result<u64> {
    let mut scores = input
        .lines()
        .map(|line| {
            if let Some(completion_string) = completion_string(line)? {
                score_completion_string(&completion_string)
            } else {
                Ok(0)
            }
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .filter(|&score| score != 0)
        .collect::<Vec<_>>();

    assert_eq!(scores.len() % 2, 1);

    scores.sort();

    Ok(scores[scores.len() / 2])
}

fn is_open_char(byte: u8) -> bool {
    matches!(byte, b'(' | b'[' | b'{' | b'<')
}

fn is_close_char(byte: u8) -> bool {
    matches!(byte, b')' | b']' | b'}' | b'>')
}

fn is_matching(open_char: u8, close_char: u8) -> Result<bool> {
    match open_char {
        b'(' => Ok(close_char == b')'),
        b'[' => Ok(close_char == b']'),
        b'{' => Ok(close_char == b'}'),
        b'<' => Ok(close_char == b'>'),
        _ => Err(anyhow!("Invalid open char: {}", open_char)),
    }
}

fn matching_close_char(open_char: u8) -> Result<u8> {
    match open_char {
        b'(' => Ok(b')'),
        b'[' => Ok(b']'),
        b'{' => Ok(b'}'),
        b'<' => Ok(b'>'),
        _ => Err(anyhow!("Invalid open char: {}", open_char)),
    }
}

fn first_illegal_close_char(line: &str) -> Result<Option<u8>> {
    let mut open_char_stack: Vec<u8> = Vec::new();

    for byte in line.bytes() {
        if is_open_char(byte) {
            open_char_stack.push(byte);
            continue;
        }

        if is_close_char(byte) {
            let Some(open_char) = open_char_stack.pop() else {
                return Ok(Some(byte));
            };

            if !is_matching(open_char, byte)? {
                return Ok(Some(byte));
            }

            continue;
        }

        return Err(anyhow!("Invalid char: {}", byte));
    }

    Ok(None)
}

fn score_illegal_close_char(close_char: u8) -> Result<u32> {
    match close_char {
        b')' => Ok(3),
        b']' => Ok(57),
        b'}' => Ok(1197),
        b'>' => Ok(25137),
        _ => Err(anyhow!("Invalid close char: {}", close_char)),
    }
}

fn completion_string(line: &str) -> Result<Option<Vec<u8>>> {
    let mut open_char_stack: Vec<u8> = Vec::new();

    for byte in line.bytes() {
        if is_open_char(byte) {
            open_char_stack.push(byte);
            continue;
        }

        if is_close_char(byte) {
            let Some(open_char) = open_char_stack.pop() else {
                return Ok(None);
            };

            if !is_matching(open_char, byte)? {
                return Ok(None);
            }

            continue;
        }

        return Err(anyhow!("Invalid char: {}", byte));
    }

    let mut close_char_stack = Vec::new();

    open_char_stack.reverse();
    for open_char in open_char_stack {
        close_char_stack.push(matching_close_char(open_char)?);
    }

    Ok(Some(close_char_stack))
}

fn score_completion_string(completion_string: &[u8]) -> Result<u64> {
    completion_string.iter().try_fold(0, |acc, close_char| {
        Ok(acc * 5 + score_completion_string_char(*close_char)?)
    })
}

fn score_completion_string_char(close_char: u8) -> Result<u64> {
    match close_char {
        b')' => Ok(1),
        b']' => Ok(2),
        b'}' => Ok(3),
        b'>' => Ok(4),
        _ => Err(anyhow!("Invalid close char: {}", close_char)),
    }
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    const EXAMPLE_INPUT: &str = r"
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(trim_newlines(EXAMPLE_INPUT))?, 26397);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(trim_newlines(EXAMPLE_INPUT))?, 288957);

        Ok(())
    }
}
