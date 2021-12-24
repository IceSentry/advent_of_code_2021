use hashbrown::HashSet;

type Data = (Vec<char>, HashSet<(isize, isize)>);

fn _print_image(image: &HashSet<(isize, isize)>) {
    let mut min_x = isize::MAX;
    let mut min_y = isize::MAX;
    let mut max_x = isize::MIN;
    let mut max_y = isize::MIN;

    for (x, y) in image {
        min_x = *x.min(&min_x);
        min_y = *y.min(&min_y);
        max_x = *x.max(&max_x);
        max_y = *y.max(&max_y);
    }

    min_x -= 1;
    min_y -= 1;
    max_x += 1;
    max_y += 1;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            print!("{}", if image.contains(&(x, y)) { '#' } else { '.' });
        }
        println!();
    }
}

fn get_enhance_index(image: &HashSet<(isize, isize)>, x: isize, y: isize, invert: bool) -> usize {
    let mut enhance_index = 0;
    for n_y in -1..=1 {
        for n_x in -1..=1 {
            enhance_index <<= 1;
            if image.contains(&(x + n_x, y + n_y)) {
                enhance_index |= invert as usize;
            } else {
                enhance_index |= !invert as usize;
            }
        }
    }
    enhance_index
}

fn enhance(
    image: &HashSet<(isize, isize)>,
    enhancer: &[char],
    i: usize,
) -> HashSet<(isize, isize)> {
    let mut next_image = HashSet::new();
    let mut visited = HashSet::new();
    let invert = enhancer[0] == '#' && i % 2 == 0;
    let match_char = if invert { '.' } else { '#' };

    for (x, y) in image {
        for dy in -1..=1 {
            for dx in -1..=1 {
                let pixel = (*x + dx, *y + dy);
                if !visited.contains(&pixel) {
                    visited.insert(pixel);
                    let enhance_index =
                        get_enhance_index(image, x + dx, y + dy, enhancer[0] != '#' || invert);
                    if enhancer[enhance_index] == match_char {
                        next_image.insert(pixel);
                    }
                }
            }
        }
    }
    next_image
}

pub fn parse(input: &str) -> Data {
    let (image_enhancement_algorithm, raw_image) = input.split_once("\n\n").unwrap();
    let mut image_set = HashSet::new();
    let image: Vec<Vec<char>> = raw_image.lines().map(|l| l.chars().collect()).collect();
    let y_offset = image.len() / 2;
    let x_offset = image[0].len() / 2;

    for (y, row) in image.iter().enumerate() {
        for (x, pixel) in row.iter().enumerate() {
            if *pixel == '#' {
                image_set.insert((
                    x as isize - x_offset as isize,
                    y as isize - y_offset as isize,
                ));
            }
        }
    }

    (image_enhancement_algorithm.chars().collect(), image_set)
}

pub fn part_1(input: &Data) -> usize {
    let (enhancer, image) = input;
    let mut next_image = image.clone();
    for i in 0..2 {
        next_image = enhance(&next_image, enhancer, i);
    }
    next_image.len()
}

pub fn part_2(input: &Data) -> usize {
    let (enhancer, image) = input;
    let mut next_image = image.clone();
    for i in 0..50 {
        next_image = enhance(&next_image, enhancer, i);
    }
    next_image.len()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const INPUTS: &str = indoc! {"
        ..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
        #..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
        .######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
        .#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
        .#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
        ...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
        ..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

        #..#.
        #....
        ##..#
        ..#..
        ..###
    "};

    pub fn parse_example() -> String {
        let (enhance, image) = INPUTS.split_once("\n\n").unwrap();
        let enhance = enhance
            .lines()
            .map(|l| l.trim())
            .collect::<Vec<_>>()
            .join("");
        format!("{}\n\n{}", enhance, image)
    }

    #[test]
    pub fn part_1() {
        let input = super::parse(&parse_example());
        let result = super::part_1(&input);
        assert_eq!(result, 35);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(&parse_example());
        let result = super::part_2(&input);
        assert_eq!(result, 3351);
    }
}
