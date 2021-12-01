use itertools::Itertools;

type Data = Vec<i32>;

pub fn parse(input: &str) -> Data {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

pub fn part_1(input: &Data) -> usize {
    let mut count = 0;
    for values in input.windows(2) {
        if values[0] < values[1] {
            count += 1;
        }
    }
    count
}

pub fn part_1_iterator(input: &Data) -> usize {
    input
        .windows(2)
        .filter(|values| values[0] < values[1])
        .count()
}

pub fn part_2(input: &Data) -> usize {
    let mut last = i32::MAX;
    let mut count = 0;
    for values in input.windows(3) {
        let sum = values.iter().sum();
        if sum > last {
            count += 1;
        }
        last = sum;
    }
    count
}

pub fn part_2_iterator(input: &Data) -> usize {
    input
        .windows(3)
        .map(|x| x.iter().sum::<i32>())
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count()
}

// (b + c) can be simplified
// a + (b + c) < (b + c) + d
pub fn part_2_iterator_2(input: &Data) -> usize {
    input
        .windows(4)
        .filter(|values| values[0] < values[3])
        .count()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const INPUTS: &str = indoc! {"
        199
        200
        208
        210
        200
        207
        240
        269
        260
        263
    "};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let result = super::part_1(&input);
        assert_eq!(result, 7);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 5);
    }
}
