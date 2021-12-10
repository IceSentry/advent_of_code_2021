use std::collections::VecDeque;

use hashbrown::HashSet;

type Data = Map;

const NEIGHBOURS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

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

    fn get_i(&self, x: isize, y: isize) -> Option<&u32> {
        if let Some(row) = self.data.get(y as usize) {
            row.get(x as usize)
        } else {
            None
        }
    }

    fn get_neighbours(&self, pos: (usize, usize)) -> Vec<((usize, usize), u32)> {
        NEIGHBOURS
            .iter()
            .map(|(n_x, n_y)| (pos.0 as isize + n_x, pos.1 as isize + n_y))
            .filter_map(|(n_x, n_y)| {
                self.get_i(n_x, n_y)
                    .map(|v| ((n_x as usize, n_y as usize), *v))
            })
            .collect()
    }

    fn check_neighbours(&self, at: (usize, usize), value: &u32) -> bool {
        NEIGHBOURS
            .iter()
            .map(|(n_x, n_y)| (at.0 as isize + n_x, at.1 as isize + n_y))
            .filter_map(|(x, y)| self.get_i(x, y))
            .all(|n| n > value)
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

pub fn part_1(input: &Data) -> u32 {
    let mut results = vec![];
    for y in 0..input.rows {
        for x in 0..input.cols {
            let value = input.data[y][x];
            if input.check_neighbours((x, y), &value) {
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
            if input.check_neighbours((x, y), &value) {
                basins.push(find_basin_size(input, (x, y)));
            }
        }
    }
    basins.sort_unstable();
    basins.iter().rev().take(3).product::<usize>() as u32
}

fn find_basin_size(map: &Map, root: (usize, usize)) -> usize {
    let mut queue = VecDeque::new();
    let mut checked = HashSet::new();
    checked.insert(root);
    queue.push_front(root);
    while !queue.is_empty() {
        let curr = queue.pop_back().expect("queue should not be empty");
        if let Some(v) = map.get(curr.0, curr.1) {
            for (n_pos, n_v) in map.get_neighbours(curr) {
                if n_v != 9 && &n_v > v && !checked.contains(&n_pos) {
                    checked.insert(n_pos);
                    queue.push_front(n_pos);
                }
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
