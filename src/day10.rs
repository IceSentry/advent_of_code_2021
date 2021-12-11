type Data = Vec<String>;

pub fn parse(input: &str) -> Data {
    input.lines().map(|l| l.to_string()).collect()
}

fn matching(c: char) -> char {
    match c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => unreachable!(),
    }
}

fn part_1_score(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

fn part_2_score(c: char) -> usize {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => unreachable!(),
    }
}

pub fn part_1(input: &Data) -> usize {
    let mut sum = 0;
    for line in input {
        let mut stack = Vec::new();
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                closing => {
                    if stack.pop().unwrap() != matching(closing) {
                        sum += match c {
                            ')' => 3,
                            ']' => 57,
                            '}' => 1197,
                            '>' => 25137,
                            _ => unreachable!(),
                        };
                        break;
                    }
                }
            }
        }
    }
    sum
}

pub fn part_2(input: &Data) -> usize {
    let mut scores = vec![];
    for line in input {
        let mut stack = Vec::new();
        let mut is_corrupted = false;
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                closing => {
                    if stack.pop().unwrap() != matching(closing) {
                        is_corrupted = true;
                        break;
                    }
                }
            }
        }

        if !is_corrupted {
            let score = stack.iter().rev().fold(0, |acc, &c| {
                acc * 5
                    + match matching(c) {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => unreachable!(),
                    }
            });
            scores.push(score)
        }
    }
    scores.sort_unstable();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const INPUTS: &str = indoc! {"
        [({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]
    "};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let result = super::part_1(&input);
        assert_eq!(result, 26397);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 288957);
    }
}
