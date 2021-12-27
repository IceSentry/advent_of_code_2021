use hashbrown::HashSet;

type Image = HashSet<u32>;
type Enhancer = Vec<u8>;
type Data = (Enhancer, Image);

fn _print_image(image: &Image) {
    let mut min_x = i16::MAX;
    let mut min_y = i16::MAX;
    let mut max_x = i16::MIN;
    let mut max_y = i16::MIN;

    for (x, y) in image.iter().map(|pixel| unpack(*pixel)) {
        min_x = x.min(min_x);
        min_y = y.min(min_y);
        max_x = x.max(max_x);
        max_y = y.max(max_y);
    }

    min_x -= 1;
    min_y -= 1;
    max_x += 1;
    max_y += 1;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let pixel = pack(x, y);
            print!("{}", if image.contains(&pixel) { '#' } else { '.' });
        }
        println!();
    }
}

fn get_enhance_index(image: &Image, x: i16, y: i16, invert: bool) -> usize {
    let mut enhance_index = 0;
    for n_y in -1..=1 {
        for n_x in -1..=1 {
            enhance_index <<= 1;
            if image.contains(&pack(x + n_x, y + n_y)) {
                enhance_index |= invert as usize;
            } else {
                enhance_index |= !invert as usize;
            }
        }
    }
    enhance_index
}

fn enhance(image: &Image, enhancer: &[u8], i: usize) -> Image {
    let mut next_image = HashSet::new();
    let mut visited = HashSet::new();
    let invert = enhancer[0] == b'#' && i % 2 == 0;
    let match_char = if invert { b'.' } else { b'#' };
    let invert = enhancer[0] != b'#' || invert;

    for (x, y) in image.iter().map(|pixel| unpack(*pixel)) {
        for dy in -1..=1 {
            for dx in -1..=1 {
                let pixel = pack(x + dx, y + dy);
                if !visited.contains(&pixel) {
                    visited.insert(pixel);
                    if enhancer[get_enhance_index(image, x + dx, y + dy, invert)] == match_char {
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
    let image: Vec<Vec<u8>> = raw_image.lines().map(|l| l.bytes().collect()).collect();
    let y_offset = image.len() / 2;
    let x_offset = image[0].len() / 2;

    for (y, row) in image.iter().enumerate() {
        for (x, pixel) in row.iter().enumerate() {
            if *pixel == b'#' {
                image_set.insert(pack(x as i16 - x_offset as i16, y as i16 - y_offset as i16));
            }
        }
    }

    (image_enhancement_algorithm.bytes().collect(), image_set)
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

fn pack(x: i16, y: i16) -> u32 {
    ((x as u32) << 16) | (y as u32 & 0x0000FFFF)
}

fn unpack(pixel: u32) -> (i16, i16) {
    (
        ((pixel & 0xFFFF0000) >> 16) as i16,
        (pixel & 0x0000FFFF) as i16,
    )
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
    pub fn pack() {
        let x_expected: i16 = 42;
        let y_expected: i16 = -42;
        let pixel = super::pack(x_expected, y_expected);
        let (x, y) = super::unpack(pixel);
        assert_eq!(x, x_expected);
        assert_eq!(y, y_expected);
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
