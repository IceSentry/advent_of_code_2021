type Data = Vec<String>;

pub fn parse(input: &str) -> Data {
    input.lines().map(|l| l.to_string()).collect()
}

fn most_common_bit(input: &Data, i: usize) -> usize {
    let count_0 = input
        .iter()
        .filter(|x| x.as_bytes().get(i) == Some(&b'0'))
        .count();
    let count_1 = input.len() - count_0;
    if count_0 > count_1 {
        0
    } else {
        1
    }
}

pub fn part_1(input: &Data) -> usize {
    let len = input[0].len();
    let mut gamma = 0;
    let mut epsilon = 0;
    for i in 0..len {
        if most_common_bit(input, i) == 0 {
            epsilon |= 1 << (len - 1 - i);
        } else {
            gamma |= 1 << (len - 1 - i);
        }
    }
    gamma * epsilon
}

pub fn part_2(input: &Data) -> usize {
    let oxygen = find(&mut input.clone(), b'0', b'1');
    let co2 = find(&mut input.clone(), b'1', b'0');
    oxygen * co2
}

fn find(input: &mut Data, a: u8, b: u8) -> usize {
    for i in 0..input[0].len() {
        if most_common_bit(input, i) == 0 {
            input.retain(|x| x.as_bytes().get(i) == Some(&a))
        } else {
            input.retain(|x| x.as_bytes().get(i) == Some(&b))
        }
        if input.len() == 1 {
            return usize::from_str_radix(&input[0], 2).unwrap();
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const INPUTS: &str = indoc! {"
        00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010
    "};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let result = super::part_1(&input);
        assert_eq!(result, 198);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 230);
    }

    #[test]
    pub fn most_common_bit() {
        let input = vec![
            "00000".into(),
            "00000".into(),
            "00000".into(),
            "00000".into(),
        ];
        let result = super::most_common_bit(&input, 0);
        assert_eq!(result, 0);
        assert_eq!("00000".to_string().as_bytes().get(0), Some(&b'0'));
    }
}
