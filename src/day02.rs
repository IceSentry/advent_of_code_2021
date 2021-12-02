use serde_scan::scan;

type Data = Vec<(String, i32)>;

pub fn parse(input: &str) -> Data {
    input
        .lines()
        .map(|line| scan!("{} {}" <- line).unwrap())
        .collect()
}

pub fn part_1(input: &Data) -> i32 {
    let mut x = 0;
    let mut y = 0;
    for (command, value) in input {
        match command.as_ref() {
            "forward" => x += value,
            "down" => y += value,
            "up" => y -= value,
            _ => panic!("unkknown command {}", command),
        }
    }
    x * y
}

pub fn part_1_iterator(input: &Data) -> i32 {
    let (x, y) = input
        .iter()
        .fold((0, 0), |(x, y), (command, value)| match command.as_ref() {
            "forward" => (x + value, y),
            "down" => (x, y + value),
            "up" => (x, y - value),
            _ => (x, y),
        });
    x * y
}

pub fn part_2(input: &Data) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    for (command, value) in input {
        match command.as_ref() {
            "forward" => {
                x += value;
                y += aim * value;
            }
            "down" => aim += value,
            "up" => aim -= value,
            _ => panic!("unkknown command {}", command),
        }
    }
    x * y
}

pub fn part_2_iterator(input: &Data) -> i32 {
    let (x, y, _aim) = input
        .iter()
        .fold((0, 0, 0), |(x, y, aim), (command, value)| {
            match command.as_ref() {
                "forward" => (x + value, y + aim * value, aim),
                "down" => (x, y, aim + value),
                "up" => (x, y, aim - value),
                _ => (x, y, aim),
            }
        });
    x * y
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const INPUTS: &str = indoc! {"
        forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2
    "};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let result = super::part_1(&input);
        assert_eq!(result, 150);
        let result = super::part_1_iterator(&input);
        assert_eq!(result, 150);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 900);
        let result = super::part_2_iterator(&input);
        assert_eq!(result, 900);
    }
}
