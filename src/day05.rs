use hashbrown::HashMap;
use serde_scan::scan;

type Data = Vec<(usize, usize, usize, usize)>;

pub fn parse(input: &str) -> Data {
    input
        .lines()
        .map(|line| scan!("{},{} -> {},{}" <- line).expect("Failed to parse input"))
        .collect()
}

pub fn part_1(input: &Data) -> usize {
    let mut map: HashMap<(usize, usize), usize> = HashMap::new();
    for (x1, y1, x2, y2) in input {
        if x1 == x2 {
            for y in *y1.min(y2)..=*y1.max(y2) {
                *map.entry((*x1, y)).or_insert(0) += 1;
            }
        } else if y1 == y2 {
            for x in *x1.min(x2)..=*x1.max(x2) {
                *map.entry((x, *y1)).or_insert(0) += 1;
            }
        }
    }
    map.values().filter(|v| **v >= 2).count()
}

pub fn part_2(input: &Data) -> usize {
    let mut map: HashMap<(usize, usize), usize> = HashMap::new();
    for (x1, y1, x2, y2) in input {
        if x1 == x2 {
            for y in *y1.min(y2)..=*y1.max(y2) {
                *map.entry((*x1, y)).or_insert(0) += 1;
            }
        } else if y1 == y2 {
            for x in *x1.min(x2)..=*x1.max(x2) {
                *map.entry((x, *y1)).or_insert(0) += 1;
            }
        } else {
            let delta_x = (*x2 as i16 - *x1 as i16).signum();
            let delta_y = (*y2 as i16 - *y1 as i16).signum();
            let mut x = *x1 as i16;
            let mut y = *y1 as i16;
            loop {
                *map.entry((x as usize, y as usize)).or_insert(0) += 1;
                if x != *x2 as i16 && y != *y2 as i16 {
                    x += delta_x;
                    y += delta_y;
                } else {
                    break;
                }
            }
        }
    }
    map.values().filter(|v| **v >= 2).count()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const INPUTS: &str = indoc! {"
        0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2
    "};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        println!("{:?}", input);

        let result = super::part_1(&input);
        assert_eq!(result, 5);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 12);
    }
}
