use std::collections::VecDeque;

use hashbrown::HashSet;

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
    let height_map: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().flat_map(|c| c.to_digit(10)).collect())
        .collect();
    let cols = height_map.len();
    let rows = height_map[0].len();
    Map {
        data: height_map,
        rows: cols,
        cols: rows,
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

const NEIGHBOURS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn check_neighbours(map: &Map, at_x: usize, at_y: usize, value: &u32) -> bool {
    NEIGHBOURS
        .iter()
        .map(|(n_x, n_y)| (at_x as isize + n_x, at_y as isize + n_y))
        .filter_map(|(x, y)| map.get(x as usize, y as usize))
        .all(|n| n > value)
}

pub fn part_1(input: &Data) -> u32 {
    let mut results = vec![];
    for y in 0..input.rows {
        for x in 0..input.cols {
            let value = input.data[y][x];
            if check_neighbours(input, x, y, &value) {
                results.push(value + 1);
            }
        }
    }
    results.iter().sum()
}

pub fn part_2(input: &Data) -> u32 {
    let mut basins = vec![];
    for y in 0..input.rows {
        for x in 0..input.cols {
            let value = input.data[y][x];
            if check_neighbours(input, x, y, &value) {
                // println!("looking for basin from ({}, {}): {}", x, y, value);
                basins.push(find_basin_size(input, x, y));
                // println!();
            }
        }
    }
    basins.sort_unstable();
    // println!("{:?}", basins);
    basins.iter().rev().take(3).product::<usize>() as u32
}

fn get_neighbours(map: &Map, x: usize, y: usize) -> Vec<((usize, usize), u32)> {
    NEIGHBOURS
        .iter()
        .map(|(n_x, n_y)| (x as isize + n_x, y as isize + n_y))
        .filter_map(|(n_x, n_y)| {
            map.get(n_x as usize, n_y as usize)
                .map(|v| ((n_x as usize, n_y as usize), *v))
        })
        .collect()
}

fn find_basin_size(map: &Map, x: usize, y: usize) -> usize {
    let mut queue = VecDeque::new();
    let mut checked = HashSet::new();
    checked.insert((x, y));
    queue.push_front((x, y));

    while !queue.is_empty() {
        let pos = queue.pop_back().expect("queue should not be empty");
        if let Some(v) = map.get(pos.0, pos.1) {
            // println!("checking neighbours of {:?}: {}", pos, v);
            for (n_pos, n_v) in get_neighbours(map, pos.0, pos.1) {
                // print!("{:?}: {}", n_pos, n_v);
                if n_v != 9 && &n_v > v && !checked.contains(&n_pos) {
                    // print!(" enqueue");
                    checked.insert(n_pos);
                    queue.push_front(n_pos);
                }
                // println!();
            }
        }
    }
    checked.len()
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
        println!("cols: {}", input.rows);
        println!("rows: {}", input.cols);

        let result = super::part_1(&input);
        assert_eq!(result, 15);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 1134);
    }
}
