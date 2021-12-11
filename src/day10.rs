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

fn complete_line(line: &str) -> Result<Vec<char>, char> {
    let mut stack = Vec::new();
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            closing => {
                if stack.pop().unwrap() != matching(closing) {
                    return Err(c);
                }
            }
        }
    }
    Ok(stack)
}

pub fn part_1(input: &Data) -> usize {
    let mut sum = 0;
    for line in input {
        if let Err(c) = complete_line(line) {
            sum += match c {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => unreachable!(),
            }
        }
    }
    sum
}

pub fn part_2(input: &Data) -> usize {
    let mut scores = vec![];
    for line in input {
        if let Ok(stack) = complete_line(line) {
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
