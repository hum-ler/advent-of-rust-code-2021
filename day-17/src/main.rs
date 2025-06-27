use std::cmp::Ordering;

use anyhow::{Result, anyhow};
use regex::Regex;

use cli::{Part, get_part};

fn main() {
    match get_part("inputs/day-17.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{error:?}"),
    }
}

fn part_1(input: &str) -> Result<i32> {
    let (_, y_range) = parse_target_area(input)?;

    // To reach max height in the trajectory, we want to "aim high" and by the time the projectile
    // is heading back down, the delta-x would already be 0 (i.e. falling straight down). So:
    //     x_lower_limit <= sum(n) <= x_upper_limit, where n = 1..=x
    // Formula is n * (n + 1) / 2.
    //
    // Assuming we start vertically with speed y, then max height is reached at:
    //     sum(n), where n = 1..=y
    // Note that this is independent of x. Once the projectile reaches max height, it starts
    // accelerating in the opposite direction following the same pattern -- by the time we return to
    // height 0, the projectile is travelling at speed y downwards.
    //
    // From here, speed will keep on increasing, so assuming the y limits are always negative:
    //     -y_upper_limit <= sum(n) <= -y_lower_limit, where n = (y + 1)..
    // =>  max_y = -y_lower_limit - 1

    let max_y = -y_range.lower - 1;

    Ok(max_y * (max_y + 1) / 2)
}

fn part_2(input: &str) -> Result<usize> {
    let (x_range, y_range) = parse_target_area(input)?;

    // Note that x and y are no longer independent. How far the projectile will travel is related
    // to step counter t.

    count_combinations(&x_range, &y_range)
}

fn parse_target_area(input: &str) -> Result<(CoordRange, CoordRange)> {
    let Some(captures) = Regex::new(
        r"x=(?<x_lower>-?\d+)..(?<x_upper>-?\d+), y=(?<y_lower>-?\d+)..(?<y_upper>-?\d+)",
    )?
    .captures(input) else {
        return Err(anyhow!("Cannot parse input with regex: {}", input));
    };

    let x_range = CoordRange {
        lower: captures["x_lower"].parse()?,
        upper: captures["x_upper"].parse()?,
    };
    let y_range = CoordRange {
        lower: captures["y_lower"].parse()?,
        upper: captures["y_upper"].parse()?,
    };

    Ok((x_range, y_range))
}

type Coord = (i32, i32);

struct CoordRange {
    lower: i32,
    upper: i32,
}

impl CoordRange {
    // The number of elements inside the range.
    fn len(&self) -> usize {
        (self.upper - self.lower + 1) as usize
    }
}

// Performs a single step, updating pos and reducing speeds where appropriate.
fn step(pos: &mut Coord, x_speed: &mut i32, y_speed: &mut i32) {
    pos.0 += *x_speed;
    pos.1 += *y_speed;

    if *x_speed > 0 {
        *x_speed -= 1;
    }
    *y_speed -= 1;
}

// Checks the relative location of pos in reference to the target area.
//
// Returns:
// - Ordering::Equal if pos is inside the target area.
// - Ordering::Less if pos is outside the target area but may still hit it.
// - Ordering::Greater if pos is outside the target area but can no longer hit it.
fn cmp_target(pos: &Coord, x_range: &CoordRange, y_range: &CoordRange) -> Ordering {
    if pos.0 >= x_range.lower
        && pos.0 <= x_range.upper
        && pos.1 >= y_range.lower
        && pos.1 <= y_range.upper
    {
        return Ordering::Equal;
    }

    if pos.0 > x_range.upper || pos.1 < y_range.lower {
        return Ordering::Greater;
    }

    Ordering::Less
}

/// Checks if the given inital speeds provide a trajectory that hits the target area.
fn hits_target(
    mut init_x: i32,
    mut init_y: i32,
    x_range: &CoordRange,
    y_range: &CoordRange,
) -> bool {
    let mut pos = (0, 0);

    loop {
        step(&mut pos, &mut init_x, &mut init_y);
        match cmp_target(&pos, x_range, y_range) {
            Ordering::Less => continue,
            Ordering::Equal => return true,
            Ordering::Greater => return false,
        }
    }
}

fn count_combinations(x_range: &CoordRange, y_range: &CoordRange) -> Result<usize> {
    // min_x occurs where speed_x drops to 0 by the time we reach the target.
    // max_x occurs when t = 1 i.e. x_range.upper.
    // min_y occurs when t = 1 i.e. y_range.lower.
    // max_y occurs at max height from part 1 i.e. -y_range.lower + 1.

    // Count combinations for t = 1.
    let mut combinations = x_range.len() * y_range.len();

    // Find min_x where speed_x = 0 when reaching target.
    let mut min_x = 1;
    while min_x * (min_x + 1) / 2 < x_range.lower {
        min_x += 1;
    }

    // Count combinations for t > 1 i.e. min_x <= x < x_range.lower.
    for x in min_x..x_range.lower {
        for y in y_range.lower..=(-y_range.lower + 1) {
            if hits_target(x, y, x_range, y_range) {
                combinations += 1;
            }
        }
    }

    Ok(combinations)
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    const EXAMPLE_INPUT: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(trim_newlines(EXAMPLE_INPUT))?, 45);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(trim_newlines(EXAMPLE_INPUT))?, 112);

        Ok(())
    }
}
