use itertools::Itertools;

type Data = Vec<i32>;

pub fn parse(input: &str) -> Data {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[allow(clippy::ptr_arg)]
pub fn part_1(input: &Data) -> usize {
    let mut count = 0;
    for values in input.windows(2) {
        if let [a, b] = values {
            if b > a {
                count += 1;
            }
        }
    }
    count
}

pub fn part_1_iterator(input: &Data) -> usize {
    input.iter().tuple_windows().filter(|(a, b)| b > a).count()
}

#[allow(clippy::ptr_arg)]
pub fn part_2(input: &Data) -> usize {
    let mut head = i32::MAX;
    let mut tail;
    let mut count = 0;
    for values in input.windows(3) {
        tail = values.iter().sum();
        if tail > head {
            count += 1;
        }
        head = tail;
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
