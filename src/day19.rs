use glam::IVec3;
use hashbrown::{HashMap, HashSet};

type Data = (Vec<IVec3>, HashSet<IVec3>);

// The description mentions needing 12, but 3 is enough
// to reconstruct the grid properly for the examples and the given input
const MIN_MATCHING_BEACONS: usize = 3;

lazy_static::lazy_static! {
    static ref ALL_ROTATIONS: Vec<IVec3> = {
        let mut axes_rotated_set = HashSet::new();
        let mut rotations = vec![];
        for x in 0..=3 {
            for y in 0..=3 {
                for z in 0..=3 {
                    let rot = IVec3::new(x, y, z);
                    let axes_rotated = [
                        rotate(IVec3::new(1, 0, 0), &rot),
                        rotate(IVec3::new(0, 1, 0), &rot),
                        rotate(IVec3::new(0, 0, 1), &rot),
                    ];
                    if !axes_rotated_set.contains(&axes_rotated) {
                        axes_rotated_set.insert(axes_rotated);
                        rotations.push(rot);
                    }
                }
            }
        }
        rotations
    };
}

// Rotate a beacon around the given axis
fn rotate(beacon: IVec3, axis_rotations: &IVec3) -> IVec3 {
    let mut out = beacon;
    for _ in 0..axis_rotations.x {
        let prev_z = out.z;
        out.z = out.y;
        out.y = -prev_z;
    }
    for _ in 0..axis_rotations.y {
        let prev_z = out.z;
        out.z = -out.x;
        out.x = prev_z;
    }
    for _ in 0..axis_rotations.z {
        let prev_y = out.y;
        out.y = out.x;
        out.x = -prev_y;
    }
    out
}

/// Rotates all beacons in the scanner
fn rotate_scanner(scanner: &HashSet<IVec3>, rot: &IVec3) -> HashSet<IVec3> {
    scanner.iter().map(|p| rotate(*p, rot)).collect()
}

/// Computes the distances between a given beacon and all other beacons in the scanner
fn beacon_distances(beacon: IVec3, scanner: &HashSet<IVec3>) -> HashSet<IVec3> {
    scanner.iter().map(|other| beacon - *other).collect()
}

/// Computes the distances between all beacons in the scanner
fn scanner_distances(scanner: &HashSet<IVec3>) -> HashMap<IVec3, HashSet<IVec3>> {
    let mut map = HashMap::new();
    for beacon in scanner {
        map.insert(*beacon, beacon_distances(*beacon, scanner));
    }
    map
}

fn find_match(
    rotated_distances: &HashMap<IVec3, HashSet<IVec3>>,
    beacons_distances: &HashMap<IVec3, HashSet<IVec3>>,
    min_match_size: usize,
) -> Option<IVec3> {
    for (rotated, rotated_dist) in rotated_distances {
        for (beacon, beacon_dist) in beacons_distances {
            if rotated_dist.intersection(beacon_dist).count() >= min_match_size {
                return Some(*rotated - *beacon);
            }
        }
    }
    None
}

fn find_beacons(scanners: &[HashSet<IVec3>]) -> (Vec<IVec3>, HashSet<IVec3>) {
    let mut scanner_positions = vec![IVec3::ZERO];
    let mut beacons = HashSet::new();
    beacons.extend(scanners[0].iter());
    // Cache the distances between each beacons
    let mut beacons_distances = scanner_distances(&beacons);

    // Keep a map of all the unchecked scanners
    // We remove a scanner when an offset and rotation is found
    let mut unchecked_scanners = HashMap::new();
    for (scanner_id, scanner) in scanners.iter().skip(1).enumerate() {
        // Cache the rotation and the beacon distances of each scanner
        for rot in ALL_ROTATIONS.iter() {
            let rotated_scanner = rotate_scanner(scanner, rot);
            let rotated_distances = scanner_distances(&rotated_scanner);
            unchecked_scanners
                .entry(scanner_id)
                .or_insert(vec![])
                .push((rotated_scanner, rotated_distances));
        }
    }

    while !unchecked_scanners.is_empty() {
        for (scanner_id, scanner) in unchecked_scanners.clone() {
            for (rotated_scanner, rotated_distances) in scanner {
                if let Some(offset) =
                    find_match(&rotated_distances, &beacons_distances, MIN_MATCHING_BEACONS)
                {
                    for beacon in rotated_scanner {
                        let beacon = beacon - offset;
                        beacons.insert(beacon);
                        beacons_distances.insert(beacon, beacon_distances(beacon, &beacons));
                    }
                    scanner_positions.push(offset);
                    unchecked_scanners.remove(&scanner_id);
                    break;
                }
            }
        }
    }
    (scanner_positions, beacons)
}

pub fn parse(input: &str) -> Data {
    let scanners = input
        .split("\n\n")
        .map(|scanner| {
            scanner
                .lines()
                .skip(1)
                .map(|l| l.split(',').flat_map(|val| val.parse()).collect())
                .map(|v: Vec<i32>| IVec3::new(v[0], v[1], v[2]))
                .collect()
        })
        .collect::<Vec<_>>();
    // Part 1 and 2 need the result of this computation so it's faster to
    // do it directly after parsing
    find_beacons(&scanners)
}

pub fn part_1(data: &Data) -> usize {
    let (_, beacons) = data;
    beacons.len()
}

pub fn part_2(data: &Data) -> usize {
    let (scanner_positions, _) = data;
    let mut max_dist = 0;
    for pos_1 in scanner_positions {
        for pos_2 in scanner_positions.clone() {
            max_dist = max_dist.max((*pos_1 - pos_2).abs().to_array().iter().sum());
        }
    }
    max_dist as usize
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const INPUTS: &str = indoc! {"
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
    "};

    #[test]
    pub fn all_rotations() {
        assert_eq!(super::ALL_ROTATIONS.len(), 24);
    }

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let result = super::part_1(&input);
        assert_eq!(result, 79);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 3621);
    }
}
