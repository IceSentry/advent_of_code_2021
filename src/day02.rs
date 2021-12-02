use serde_derive::Deserialize;
use serde_scan::scan;

type Data = Vec<(Commands, i32)>;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Commands {
    Forward,
    Down,
    Up,
}

pub fn parse(input: &str) -> Data {
    input
        .lines()
        .map(|line| scan!("{} {}" <- line).expect("Failed to parse input"))
        .collect()
}

pub fn part_1(input: &Data) -> i32 {
    let mut x = 0;
    let mut y = 0;
    for (command, value) in input {
        match command {
            Commands::Forward => x += value,
            Commands::Down => y += value,
            Commands::Up => y -= value,
        }
    }
    x * y
}

pub fn part_1_iterator(input: &Data) -> i32 {
    let (x, y) = input
        .iter()
        .fold((0, 0), |(x, y), (command, value)| match command {
            Commands::Forward => (x + value, y),
            Commands::Down => (x, y + value),
            Commands::Up => (x, y - value),
        });
    x * y
}

pub fn part_2(input: &Data) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    for (command, value) in input {
        match command {
            Commands::Forward => {
                x += value;
                y += aim * value;
            }
            Commands::Down => aim += value,
            Commands::Up => aim -= value,
        }
    }
    x * y
}

pub fn part_2_iterator(input: &Data) -> i32 {
    let (x, y, _aim) =
        input
            .iter()
            .fold((0, 0, 0), |(x, y, aim), (command, value)| match command {
                Commands::Forward => (x + value, y + aim * value, aim),
                Commands::Down => (x, y, aim + value),
                Commands::Up => (x, y, aim - value),
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
