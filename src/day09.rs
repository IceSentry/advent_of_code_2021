use std::collections::VecDeque;

use hashbrown::HashSet;

type Data = Map;

const NEIGHBOURS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

#[derive(Clone)]
pub struct Map {
    data: Vec<Vec<u32>>,
}

impl Map {
    fn get(&self, pos: (isize, isize)) -> Option<&u32> {
        self.data
            .get(pos.1 as usize)
            .and_then(|row| row.get(pos.0 as usize))
    }

    fn get_neighbours(&self, pos: (usize, usize)) -> Vec<((usize, usize), u32)> {
        NEIGHBOURS
            .iter()
            .map(|(n_x, n_y)| (pos.0 as isize + n_x, pos.1 as isize + n_y))
            .filter_map(|n| self.get(n).map(|v| ((n.0 as usize, n.1 as usize), *v)))
            .collect()
    }

    fn check_neighbours(&self, pos: (usize, usize), value: &u32) -> bool {
        NEIGHBOURS
            .iter()
            .map(|(n_x, n_y)| (pos.0 as isize + n_x, pos.1 as isize + n_y))
            .filter_map(|n| self.get(n))
            .all(|n| n > value)
    }
}

pub fn parse(input: &str) -> Data {
    let height_map: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().flat_map(|c| c.to_digit(10)).collect())
        .collect();
    Map { data: height_map }
}

pub fn part_1(input: &Data) -> u32 {
    let mut results = vec![];
    for (y, rows) in input.data.iter().enumerate() {
        for (x, value) in rows.iter().enumerate() {
            if input.check_neighbours((x, y), value) {
                results.push(value + 1);
            }
        }
    }
    results.iter().sum()
}

pub fn part_2(input: &Data) -> u32 {
    let mut basins = vec![0; 3];
    for (y, rows) in input.data.iter().enumerate() {
        for (x, value) in rows.iter().enumerate() {
            if input.check_neighbours((x, y), value) {
                let size = find_basin_size(input, (x, y));
                if basins[0] < size {
                    basins[2] = basins[1];
                    basins[1] = basins[0];
                    basins[0] = size;
                } else if basins[1] < size {
                    basins[2] = basins[1];
                    basins[1] = size;
                } else if basins[2] < size {
                    basins[2] = size;
                }
            }
        }
    }
    basins.iter().product::<usize>() as u32
}

fn find_basin_size(map: &Map, root: (usize, usize)) -> usize {
    let mut queue = VecDeque::new();
    let mut checked = HashSet::new();
    checked.insert(root);
    queue.push_back(root);
    while !queue.is_empty() {
        let curr = queue.pop_front().expect("queue should not be empty");
        let v = map.data[curr.1][curr.0];
        for (n_pos, n_v) in map.get_neighbours(curr) {
            if n_v != 9 && n_v > v && !checked.contains(&n_pos) {
                checked.insert(n_pos);
                queue.push_back(n_pos);
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
