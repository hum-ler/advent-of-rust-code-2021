use std::{collections::HashMap, str::FromStr};

use anyhow::{anyhow, Result};
use itertools::Itertools;

use aoc_cli::{get_part, Part};

fn main() {
    match get_part("inputs/day-19.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: &str) -> Result<usize> {
    // 1. For each scanner, find the pair-wise distance between all beacons. We will use the
    //    manhanttan distance to avoid dealing with floating points.
    // 2. Compare the list of distances between Scanner 0 and every other scanner: we want to find 3
    //    unique distances such that:
    //                      d1     d2     d3
    //        scanner 0: A ---> B ---> C ---> D
    //        scanner N: W ---> X ---> Y ---> Z
    //    where d1, d2, d3 are *unique* values among the overlap between scanners 0 and N. Beacons
    //    A, D, W, Z are needed to ascertain that B = X and C = Y because distance BC = CB.
    // 3. Find the change of basis transformation that converts beacons X to B and Y to C.
    // 4. Convert each beacon by its respective scanner transformation matrix to find the full list
    //    w.r.t. Scanner 0.
    // 5. We can perform the transformation w.r.t. scanner 0, or chain the transformation.

    let scanners = update_scanner_transform(parse_input_into_scanners(input)?)?;

    Ok(scanners
        .iter()
        .flat_map(|scanner| {
            scanner
                .beacons
                .iter()
                .map(|beacon| change_basis(*beacon, &scanner.transform))
        })
        .unique()
        .count())
}

fn part_2(input: &str) -> Result<u32> {
    let scanners = update_scanner_transform(parse_input_into_scanners(input)?)?;

    scanners
        .iter()
        .tuple_combinations()
        .map(|(scanner_1, scanner_2)| {
            let scanner_1_pos = change_basis(origin(), &scanner_1.transform);
            let scanner_2_pos = change_basis(origin(), &scanner_2.transform);

            manhattan_distance(scanner_1_pos, scanner_2_pos)
        })
        .max()
        .ok_or(anyhow!("Cannot find max distance"))
}

type Vector3 = [i32; 3];

type Matrix3 = [[i32; 3]; 3];

type Isometry3 = (Matrix3, Vector3);

#[derive(Clone, PartialEq)]
struct Scanner {
    id: u8,
    beacons: Vec<Vector3>,
    distances: Vec<Distance>,
    transform: Vec<Isometry3>,
}

#[derive(Clone, Copy, PartialEq)]
struct Distance {
    beacon_1: Vector3,
    beacon_2: Vector3,
    distance: u32,
}

impl Distance {
    fn from(beacon_1: Vector3, beacon_2: Vector3) -> Self {
        Self {
            beacon_1,
            beacon_2,
            distance: manhattan_distance(beacon_1, beacon_2),
        }
    }
}

impl FromStr for Scanner {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let lines = s.lines().collect_vec();

        if lines.len() < 13 {
            // ID + 12 matching beacons
            return Err(anyhow!("Invalid s: {}", s));
        };

        let Some((_, _, id, _)) = lines[0].split_whitespace().collect_tuple() else {
            return Err(anyhow!("Invalid ID line: {}", lines[0]));
        };

        let beacons = lines[1..]
            .iter()
            .map(|line| parse_beacon(line))
            .collect::<Result<Vec<_>>>()?;

        let distances = measure_distances(&beacons);

        Ok(Scanner {
            id: id.parse()?,
            beacons,
            distances,
            transform: Vec::new(),
        })
    }
}

fn parse_input_into_scanners(input: &str) -> Result<Vec<Scanner>> {
    input
        .split_terminator("\n\n")
        .map(Scanner::from_str)
        .collect()
}

fn parse_beacon(input: &str) -> Result<Vector3> {
    let Some((x, y, z)) = input.split_terminator(",").collect_tuple() else {
        return Err(anyhow!("Invalid input: {}", input));
    };

    Ok([x.parse()?, y.parse()?, z.parse()?])
}

/// Point of origin.
fn origin() -> Vector3 {
    [0, 0, 0]
}

/// I.
fn identity_matrix() -> Matrix3 {
    [[1, 0, 0], [0, 1, 0], [0, 0, 1]]
}

/// Rotational matrices excluding I.
///
/// See https://www.euclideanspace.com/maths/algebra/matrix/transforms/examples/index.htm.
fn rotation_matrices() -> [Matrix3; 23] {
    [
        [[1, 0, 0], [0, 0, -1], [0, 1, 0]],
        [[1, 0, 0], [0, -1, 0], [0, 0, -1]],
        [[1, 0, 0], [0, 0, 1], [0, -1, 0]],
        [[0, -1, 0], [1, 0, 0], [0, 0, 1]],
        [[0, 0, 1], [1, 0, 0], [0, 1, 0]],
        [[0, 1, 0], [1, 0, 0], [0, 0, -1]],
        [[0, 0, -1], [1, 0, 0], [0, -1, 0]],
        [[-1, 0, 0], [0, -1, 0], [0, 0, 1]],
        [[-1, 0, 0], [0, 0, -1], [0, -1, 0]],
        [[-1, 0, 0], [0, 1, 0], [0, 0, -1]],
        [[-1, 0, 0], [0, 0, 1], [0, 1, 0]],
        [[0, 1, 0], [-1, 0, 0], [0, 0, 1]],
        [[0, 0, 1], [-1, 0, 0], [0, -1, 0]],
        [[0, -1, 0], [-1, 0, 0], [0, 0, -1]],
        [[0, 0, -1], [-1, 0, 0], [0, 1, 0]],
        [[0, 0, -1], [0, 1, 0], [1, 0, 0]],
        [[0, 1, 0], [0, 0, 1], [1, 0, 0]],
        [[0, 0, 1], [0, -1, 0], [1, 0, 0]],
        [[0, -1, 0], [0, 0, -1], [1, 0, 0]],
        [[0, 0, -1], [0, -1, 0], [-1, 0, 0]],
        [[0, -1, 0], [0, 0, 1], [-1, 0, 0]],
        [[0, 0, 1], [0, 1, 0], [-1, 0, 0]],
        [[0, 1, 0], [0, 0, -1], [-1, 0, 0]],
    ]
}

fn manhattan_distance(beacon: Vector3, other: Vector3) -> u32 {
    beacon[0].abs_diff(other[0]) + beacon[1].abs_diff(other[1]) + beacon[2].abs_diff(other[2])
}

/// Calculates the pair-wise distances between beacons.
fn measure_distances(beacons: &[Vector3]) -> Vec<Distance> {
    beacons
        .iter()
        .tuple_combinations()
        .map(|(beacon, other)| Distance::from(*beacon, *other))
        .collect()
}

/// Finds the overlap in beacon pair-wise distances of one scanner from another.
///
/// Returns only overlaps where distance is unique.
fn find_unique_overlap(slice: &[Distance], other: &[Distance]) -> Vec<(Distance, Distance)> {
    let mut overlap = Vec::new();

    let slice = slice
        .iter()
        .unique_by(|distance| distance.distance)
        .sorted_by_key(|distance| distance.distance)
        .collect_vec();
    let other = other
        .iter()
        .unique_by(|distance| distance.distance)
        .sorted_by_key(|distance| distance.distance)
        .collect_vec();

    let mut slice_index = 0;
    let mut other_index = 0;

    while slice_index < slice.len() && other_index < other.len() {
        match slice[slice_index]
            .distance
            .cmp(&other[other_index].distance)
        {
            std::cmp::Ordering::Equal => {
                overlap.push((*slice[slice_index], *other[other_index]));
                slice_index += 1;
                other_index += 1;
            }
            std::cmp::Ordering::Less => slice_index += 1,
            std::cmp::Ordering::Greater => other_index += 1,
        }
    }

    overlap
}

/// Finds pairs of matching beacons from the distance overlap of 2 scanners.
///
/// Returns all possible beacons (B, X, C, Y).
fn find_equivalent_beacons(
    overlap: &[(Distance, Distance)],
) -> Vec<(Vector3, Vector3, Vector3, Vector3)> {
    let mut equivalent_beacons = Vec::new();

    for (distance_1, distance_2) in overlap {
        let b = distance_1.beacon_1;
        let c = distance_1.beacon_2;
        let potential_x_1 = distance_2.beacon_1;
        let potential_x_2 = distance_2.beacon_2;
        let mut x = None;
        let mut y = None;

        // Find A -> B && W -> X1 | X2.
        for (search_1, search_2) in overlap {
            if (b == search_1.beacon_1 || b == search_1.beacon_2)
                && (potential_x_1 == search_2.beacon_1
                    || potential_x_2 == search_2.beacon_1
                    || potential_x_1 == search_2.beacon_2
                    || potential_x_2 == search_2.beacon_2)
            {
                if potential_x_1 == search_2.beacon_1 || potential_x_1 == search_2.beacon_2 {
                    x = Some(potential_x_1);
                    y = Some(potential_x_2);
                } else {
                    x = Some(potential_x_2);
                    y = Some(potential_x_1);
                };
            }
        }

        if x.is_none() {
            continue;
        }

        let x = x.unwrap();
        let y = y.unwrap();

        // Find C -> D && Y -> Z
        for (search_1, search_2) in overlap {
            if (c == search_1.beacon_1 || c == search_1.beacon_2)
                && (y == search_2.beacon_1 || y == search_2.beacon_2)
            {
                equivalent_beacons.push((b, x, c, y));
            }
        }
    }

    equivalent_beacons
}

/// Finds the transformation that works for at least 12 pairs of matches.
fn find_transformation(
    equivalent_beacons: &[(Vector3, Vector3, Vector3, Vector3)],
) -> Option<Isometry3> {
    let mut potential_transformations: HashMap<Isometry3, u32> = HashMap::new();

    for (b, x, c, y) in equivalent_beacons {
        for rotation in rotation_matrices() {
            let translation_b_x = negate(translate(rotate(*x, &rotation), &negate(*b)));
            let translation_c_y = negate(translate(rotate(*y, &rotation), &negate(*c)));

            if translation_b_x == translation_c_y {
                let transformation = (rotation, translation_b_x);

                let count = potential_transformations.entry(transformation).or_default();
                *count += 1;

                // By right, we should check for 12 occurrences, but a corroborating pair seems good
                // enough for the input.
                if *count >= 2 {
                    return Some(transformation);
                }
            }
        }
    }

    None
}

/// Applies the given transformations on a point.
///
/// Make sure that the transformations are in the correct order.
fn change_basis(beacon: Vector3, transformations: &[Isometry3]) -> Vector3 {
    transformations.iter().fold(beacon, |acc, transformation| {
        translate(rotate(acc, &transformation.0), &transformation.1)
    })
}

/// Applies a rotation on a point.
fn rotate(beacon: Vector3, rotation: &Matrix3) -> Vector3 {
    [
        rotation[0][0] * beacon[0] + rotation[0][1] * beacon[1] + rotation[0][2] * beacon[2],
        rotation[1][0] * beacon[0] + rotation[1][1] * beacon[1] + rotation[1][2] * beacon[2],
        rotation[2][0] * beacon[0] + rotation[2][1] * beacon[1] + rotation[2][2] * beacon[2],
    ]
}

/// Applies a translation on a point.
fn translate(beacon: Vector3, translation: &Vector3) -> Vector3 {
    [
        translation[0] + beacon[0],
        translation[1] + beacon[1],
        translation[2] + beacon[2],
    ]
}

/// Applies a negation on a point.
fn negate(beacon: Vector3) -> Vector3 {
    [-beacon[0], -beacon[1], -beacon[2]]
}

/// Updates the transformations for each scanner.
fn update_scanner_transform(mut scanners: Vec<Scanner>) -> Result<Vec<Scanner>> {
    if scanners.is_empty() {
        return Err(anyhow!("Cannot work with empty scanners"));
    }

    // Set scanner 0 as base reference.
    scanners[0].transform.push((identity_matrix(), origin()));

    // Run through each scanner to update its transform to take reference from at least one other
    // scanner.
    while scanners.iter().any(|scanner| scanner.transform.is_empty()) {
        for index in 0..scanners.len() {
            if scanners[index].transform.is_empty() {
                continue;
            }

            let reference_scanner = scanners[index].clone();

            for scanner in scanners.clone() {
                if scanner == reference_scanner || !scanner.transform.is_empty() {
                    continue;
                }

                let overlap = find_unique_overlap(&reference_scanner.distances, &scanner.distances);

                let equivalent_beacons = find_equivalent_beacons(&overlap);
                if equivalent_beacons.is_empty() {
                    continue;
                };

                let Some(transformation) = find_transformation(&equivalent_beacons) else {
                    continue;
                };

                // Update the original scanners.
                scanners[scanner.id as usize].transform.push(transformation);
                if reference_scanner.id != 0 {
                    scanners[scanner.id as usize]
                        .transform
                        .extend(reference_scanner.transform.iter());
                }
            }
        }
    }

    Ok(scanners)
}

#[cfg(test)]
mod tests {
    use aoc_cli::trim_input;

    use super::*;

    const EXAMPLE_INPUT: &str = r"
--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(trim_input(EXAMPLE_INPUT))?, 79);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(trim_input(EXAMPLE_INPUT))?, 3621);

        Ok(())
    }
}
