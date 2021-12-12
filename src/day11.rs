use colored::Colorize;

type Data = Vec<Vec<i32>>;

pub fn parse(input: &str) -> Data {
    input
        .lines()
        .map(|l| {
            l.chars()
                .flat_map(|c| c.to_digit(10))
                .map(|x| x as i32)
                .collect()
        })
        .collect()
}

const NEIGHBOURS: [(isize, isize); 8] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (1, 1),
    (1, -1),
    (-1, 0),
    (-1, 1),
    (-1, -1),
];

fn _print_data(data: &Data) {
    for row in data {
        for val in row {
            if *val == 0 {
                print!("{}", val.to_string().yellow());
            } else {
                print!("{}", val);
            }
        }
        println!();
    }
    println!();
}

fn step(data: &mut Data) -> usize {
    for rows in data.iter_mut() {
        for value in rows.iter_mut() {
            *value += 1;
        }
    }

    let mut flashes = 0;
    loop {
        let mut keeo_running = false;
        for y in 0..10 {
            for x in 0..10 {
                if data[y][x] <= 9 {
                    continue;
                }

                flashes += 1;
                keeo_running = true;
                data[y][x] = 0;

                for (n_x, n_y) in NEIGHBOURS {
                    if let Some(val) = data
                        .get_mut((y as isize + n_y) as usize)
                        .and_then(|row| row.get_mut((x as isize + n_x) as usize))
                    {
                        if *val <= 9 && *val != 0 {
                            *val += 1;
                        }
                    }
                }
            }
        }
        if !keeo_running {
            return flashes;
        }
    }
}

pub fn part_1(input: &Data) -> usize {
    let mut data = input.clone();
    let mut flashes = 0;
    for _ in 0..100 {
        flashes += step(&mut data);
    }
    flashes
}

pub fn part_2(input: &Data) -> usize {
    let mut data = input.clone();
    let mut i = 0;
    loop {
        i += 1;
        // step(&mut data);
        if step(&mut data) == 100 {
            return i;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day11::step;
    use indoc::indoc;

    const INPUTS: &str = indoc! {"
        5483143223
        2745854711
        5264556173
        6141336146
        6357385478
        4167524645
        2176841721
        6882881134
        4846848554
        5283751526
    "};

    #[test]
    pub fn step_test() {
        let mut input = super::parse(INPUTS);
        let mut flashes = 0;
        for _ in 0..10 {
            flashes += step(&mut input);
        }
        assert_eq!(flashes, 204);
    }

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let result = super::part_1(&input);
        assert_eq!(result, 1656);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 195);
    }
}
