/// See this paper for more detail on the median and mean solution
/// https://www.reddit.com/r/adventofcode/comments/rawxad/2021_day_7_part_2_i_wrote_a_paper_on_todays/

type Data = Vec<isize>;

pub fn parse(input: &str) -> Data {
    input
        .trim_end()
        .split(',')
        .map(|l| l.parse().unwrap())
        .collect()
}

pub fn part_1(input: &Data) -> usize {
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
    let mean = input.iter().sum::<isize>() as f32 / input.len() as f32;
    let fuel = |dist: isize| (dist.pow(2) + dist) / 2;

    let sum_1: isize = input
        .iter()
        .map(|crab| fuel((crab - mean.floor() as isize).abs()))
        .sum();

    let sum_2: isize = input
        .iter()
        .map(|crab| fuel((crab - mean.ceil() as isize).abs()))
        .sum();

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
