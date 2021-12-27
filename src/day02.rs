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
    let mut horizontal = 0;
    let mut depth = 0;
    for (command, value) in input {
        match command {
            Commands::Forward => horizontal += value,
            Commands::Down => depth += value,
            Commands::Up => depth -= value,
        }
    }
    horizontal * depth
}

pub fn part_2(input: &Data) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for (command, value) in input {
        match command {
            Commands::Forward => {
                horizontal += value;
                depth += aim * value;
            }
            Commands::Down => aim += value,
            Commands::Up => aim -= value,
        }
    }
    horizontal * depth
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
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 900);
    }
}
