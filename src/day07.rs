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

pub fn part_1_median(input: &Data) -> usize {
    let mut input = input.clone();
    input.sort_unstable();
    let median = input[(input.len() / 2) as usize];
    let mut sum = 0;
    for crab in input {
        sum += (crab - median).abs();
    }
    sum as usize
}

pub fn part_2(input: &Data) -> usize {
    let max = input.iter().max().unwrap();
    let mut vec = vec![];
    for i in 0..*max {
        vec.push(0);
        for crab in input {
            let dist = *crab - i;
            let fuel = (dist.pow(2) + dist.abs()) / 2;
            vec[i as usize] += fuel;
        }
    }
    *vec.iter().min().unwrap() as usize
}

pub fn part_2_mean(input: &Data) -> usize {
    let n = input.len() as isize;
    let mean = input.iter().sum::<isize>() / n;
    let fuel = |dist: isize| (dist.pow(2) + dist.abs()) / 2;

    let mut sum_1 = 0;
    for crab in input {
        let dist = (crab - (mean as f32).floor() as isize).abs();
        sum_1 += fuel(dist);
    }

    let mut sum_2 = 0;
    for crab in input {
        let dist = (crab - (mean as f32).ceil() as isize).abs();
        sum_2 += fuel(dist);
    }

    sum_1.min(sum_2) as usize
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
