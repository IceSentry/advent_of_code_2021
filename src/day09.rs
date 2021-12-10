type Data = Map;

pub struct Map {
    data: Vec<Vec<u32>>,
    cols: usize,
    rows: usize,
}

impl Map {
    fn get(&self, x: usize, y: usize) -> Option<&u32> {
        if let Some(row) = self.data.get(y) {
            row.get(x)
        } else {
            None
        }
    }
}

pub fn parse(input: &str) -> Data {
    let mut lines = input.trim().lines().peekable();
    let cols = lines.peek().unwrap().len();
    let rows = lines.count();
    let height_map = input
        .lines()
        .map(|l| l.chars().flat_map(|c| c.to_digit(10)).collect())
        .collect();
    Map {
        data: height_map,
        cols,
        rows,
    }
}

// 1D array version doesn't work for some reason
// fn check_neighbours(map: &Map, at: usize, value: &u32) -> bool {
//     let mut neighbours: Vec<isize> = vec![map.cols as isize, -(map.cols as isize)];
//     let div = at / map.cols;
//     if div == map.cols {
//         neighbours.push(-1);
//     } else if div == 0 {
//         neighbours.push(1);
//     } else {
//         neighbours.push(1);
//         neighbours.push(-1);
//     }
//     neighbours
//         .iter()
//         .map(|offset| at as isize + offset)
//         .filter_map(|index| map.data.get(index as usize))
//         .all(|n| n > value)
// }

// pub fn part_1(input: &Data) -> u32 {
//     let mut results = vec![];
//     for (i, value) in input.data.iter().enumerate() {
//         if check_neighbours(input, i, value) {
//             results.push(value + 1);
//         }
//     }
//     results.iter().sum()
// }

fn check_neighbours(map: &Map, at_x: usize, at_y: usize, value: u32) -> bool {
    let neighbours: Vec<(isize, isize)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    neighbours
        .iter()
        .map(|(n_x, n_y)| (at_x as isize + n_x, at_y as isize + n_y))
        .filter_map(|(x, y)| map.get(x as usize, y as usize))
        .all(|n| n > &value)
}

pub fn part_1(input: &Data) -> u32 {
    let mut results = vec![];
    for y in 0..input.cols {
        for x in 0..input.rows {
            let value = input.data[y][x];
            if check_neighbours(input, x, y, value) {
                results.push(value + 1);
            }
        }
    }
    results.iter().sum()
}

pub fn part_2(input: &Data) -> u32 {
    for y in 0..input.cols {
        for x in 0..input.rows {
            let value = input.data[y][x];
            if check_neighbours(input, x, y, value) {
                // TODO start BFS
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const INPUTS: &str = indoc! {"
        2199943210
        3987894921
        9856789892
        8767896789
        9899965678
    "};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        println!("{:?}", input.data);
        println!("{:?}", input.data.len());
        println!("cols: {}", input.cols);

        let result = super::part_1(&input);
        assert_eq!(result, 15);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 0);
    }
}
