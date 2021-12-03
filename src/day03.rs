type Data = Vec<Vec<char>>;

pub fn parse(input: &str) -> Data {
    input.lines().map(|l| l.chars().collect()).collect()
}

pub fn part_1(input: &Data) -> usize {
    let len = input.first().unwrap().len();
    let mut gamma = 0;
    let mut epsilon = 0;
    for i in 0..len {
        let count_0 = input.iter().filter(|x| x[i] == '0').count();
        let count_1 = input.len() - count_0;
        if count_0 > count_1 {
            // most common bit is 0
            // least common bit is 1
            epsilon += 1 << (len - 1 - i);
        } else {
            // most common bit is 1
            // least common bit is 0
            gamma += 1 << (len - 1 - i);
        }
    }
    gamma * epsilon
}

pub fn part_2(input: &Data) -> usize {
    let oxygen = find(&mut input.clone(), '0', '1');
    let co2 = find(&mut input.clone(), '1', '0');
    oxygen * co2
}

fn find(input: &mut Data, a: char, b: char) -> usize {
    let len = input.first().unwrap().len();
    for i in 0..len {
        let count_0 = input.iter().filter(|x| x[i] == '0').count();
        let count_1 = input.len() - count_0;
        if count_0 > count_1 {
            input.retain(|x| x[i] == a)
        } else {
            input.retain(|x| x[i] == b)
        }
        if input.len() == 1 {
            return usize::from_str_radix(&String::from_iter(input[0].clone()), 2).unwrap();
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
}
