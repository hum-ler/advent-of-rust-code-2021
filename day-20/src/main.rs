use anyhow::{Result, anyhow};

use cli::{Part, get_part};

fn main() {
    match get_part("inputs/day-20.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{error:?}"),
    }
}

fn part_1(input: &str) -> Result<usize> {
    let (algorithm, mut image) = parse_input_into_algorithm_and_image(input)?;

    // The top-left pixel of the image is the bottom-right pixel of the 9-square. So we have to
    // expand twice initially.
    //
    // Also, for the actual input, the 0-th pixel in algorithm is Light, while the 512-nd pixel is
    // Dark, meaning that each time we enhance, the background will toggle between light and dark.

    image = enhance_image(image, Pixel::Dark, &algorithm)?;
    image = enhance_image(image, algorithm[0], &algorithm)?;

    Ok(image
        .into_iter()
        .map(|row| {
            row.into_iter()
                .filter(|pixel| *pixel == Pixel::Light)
                .count()
        })
        .sum())
}

fn part_2(input: &str) -> Result<usize> {
    let (algorithm, mut image) = parse_input_into_algorithm_and_image(input)?;

    let toggle_background = algorithm[0] == Pixel::Light && algorithm[511] == Pixel::Dark;
    let light_background = algorithm[0] == Pixel::Light && algorithm[511] == Pixel::Light;

    for index in 0..50 {
        let expand_with =
            if (light_background && index != 0) || (toggle_background && index % 2 == 1) {
                Pixel::Light
            } else {
                Pixel::Dark
            };

        image = enhance_image(image, expand_with, &algorithm)?;
    }

    Ok(image
        .into_iter()
        .map(|row| {
            row.into_iter()
                .filter(|pixel| *pixel == Pixel::Light)
                .count()
        })
        .sum())
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
enum Pixel {
    Dark,
    Light,
}

fn parse_input_into_algorithm_and_image(input: &str) -> Result<(Vec<Pixel>, Vec<Vec<Pixel>>)> {
    let Some((algorithm_part, image_part)) = input.split_once("\n\n") else {
        return Err(anyhow!(
            "Cannot split input into algorithm and image: {}",
            input
        ));
    };

    let algorithm = parse_pixels(algorithm_part)?;

    let image = image_part
        .lines()
        .map(parse_pixels)
        .collect::<Result<Vec<_>>>()?;

    Ok((algorithm, image))
}

fn parse_pixels(input: &str) -> Result<Vec<Pixel>> {
    input
        .bytes()
        .map(|byte| match byte {
            b'.' => Ok(Pixel::Dark),
            b'#' => Ok(Pixel::Light),
            x => Err(anyhow!("Invalid pixel: {}", x)),
        })
        .collect()
}

/// Expands the image area by a 1-pixel border, using expand_with.
fn expand_image(mut image: Vec<Vec<Pixel>>, expand_with: Pixel) -> Vec<Vec<Pixel>> {
    image.push(vec![expand_with; image[0].len()]);
    image.push(vec![expand_with; image[0].len()]);
    image.rotate_right(1);

    image
        .into_iter()
        .map(|mut row| {
            row.push(expand_with);
            row.push(expand_with);
            row.rotate_right(1);

            row
        })
        .collect()
}

/// Shrinks the image area by removing a 1-pixel border.
fn shrink_image(mut image: Vec<Vec<Pixel>>) -> Vec<Vec<Pixel>> {
    image.rotate_left(1);
    image.pop();
    image.pop();

    image
        .into_iter()
        .map(|mut row| {
            row.rotate_left(1);
            row.pop();
            row.pop();

            row
        })
        .collect()
}

/// Enhances the image.
fn enhance_image(
    mut image: Vec<Vec<Pixel>>,
    expand_with: Pixel,
    algorithm: &[Pixel],
) -> Result<Vec<Vec<Pixel>>> {
    image = expand_image(image, expand_with);
    image = expand_image(image, expand_with);

    let orig_image = image.clone();

    for row in 1..image.len() - 1 {
        for col in 1..image[0].len() - 1 {
            let index = [
                orig_image[row - 1][col - 1],
                orig_image[row - 1][col],
                orig_image[row - 1][col + 1],
                orig_image[row][col - 1],
                orig_image[row][col],
                orig_image[row][col + 1],
                orig_image[row + 1][col - 1],
                orig_image[row + 1][col],
                orig_image[row + 1][col + 1],
            ]
            .iter()
            .map(|pixel| *pixel as usize)
            .reduce(|acc, pixel| (acc << 1) + pixel)
            .ok_or(anyhow!("Cannot reduce pixels to usize"))?;

            image[row][col] = algorithm[index];
        }
    }

    image = shrink_image(image);

    Ok(image)
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    const EXAMPLE_INPUT: &str = r"
..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(trim_newlines(EXAMPLE_INPUT))?, 35);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(trim_newlines(EXAMPLE_INPUT))?, 3351);

        Ok(())
    }
}
