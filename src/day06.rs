type Data = Vec<u8>;

pub fn parse(input: &str) -> Data {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|l| l.parse().unwrap())
        .collect()
}

fn compute_fish(input: &Data, n: usize) -> usize {
    let mut fishes = vec![0; 9];
    for fish in input {
        fishes[*fish as usize] += 1;
    }

    for _ in 0..n {
        let new_fish = fishes[0];
        fishes.rotate_left(1);
        fishes[6] += new_fish;
    }
    fishes.iter().sum()
}

pub fn part_1(input: &Data) -> usize {
    compute_fish(input, 80)
}

pub fn part_2(input: &Data) -> usize {
    compute_fish(input, 256)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const INPUTS: &str = indoc! {"
        3,4,3,1,2
    "};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let result = super::part_1(&input);
        assert_eq!(result, 5934);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 26984457539);
    }
}
