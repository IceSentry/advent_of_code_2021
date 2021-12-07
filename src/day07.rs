type Data = Vec<isize>;

pub fn parse(input: &str) -> Data {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|l| l.parse().unwrap())
        .collect()
}

pub fn part_1(input: &Data) -> usize {
    let max = input.iter().max().unwrap();
    let mut vec = vec![];
    for i in 0..*max {
        vec.push(0);
        for crab in input {
            vec[i as usize] += (*crab - i).abs();
        }
    }
    *vec.iter().min().unwrap() as usize
}

pub fn part_2(input: &Data) -> usize {
    let max = input.iter().max().unwrap();
    let mut vec = vec![];
    for i in 0..*max {
        vec.push(0);
        for crab in input {
            let dist = (*crab - i).abs();
            // memoizing this is way slower because most numbers are not repeated
            vec[i as usize] += dist * (dist + 1) / 2;
        }
    }
    *vec.iter().min().unwrap() as usize
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const INPUTS: &str = indoc! {"
        16,1,2,0,4,2,7,1,2,14
    "};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let result = super::part_1(&input);
        assert_eq!(result, 37);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 168);
    }
}
