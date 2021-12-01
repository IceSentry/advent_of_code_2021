type Data = Vec<i32>;

pub fn parse(input: &str) -> Data {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[allow(clippy::ptr_arg)]
pub fn part_1(input: &Data) -> usize {
    let mut last = 0;
    let mut count = 0;
    for &value in input.iter().skip(1) {
        if value > last {
            count += 1;
        }
        last = value;
    }
    count
}

#[allow(clippy::ptr_arg)]
pub fn part_2(input: &Data) -> usize {
    let mut head = 0;
    let mut tail = 0;
    let mut count = 0;
    for (i, values) in input.windows(3).enumerate() {
        tail = values.iter().sum();
        if i > 0 && tail > head {
            count += 1;
        }
        head = tail;
    }
    count
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
