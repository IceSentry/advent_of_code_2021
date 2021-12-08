use itertools::Itertools;

type Data = Vec<(Vec<Vec<char>>, Vec<Vec<char>>)>;

pub fn parse(input: &str) -> Data {
    input
        .lines()
        .map(|line| line.split_once('|').unwrap())
        .map(|(l, r)| {
            (
                l.trim()
                    .split(' ')
                    .map(|s| s.chars().sorted().collect())
                    .collect(),
                r.trim()
                    .split(' ')
                    .map(|s| s.chars().sorted().collect())
                    .collect(),
            )
        })
        .collect()
}

pub fn part_1(input: &Data) -> usize {
    input
        .iter()
        .map(|(_, outputs)| {
            outputs
                .iter()
                .filter(|x| matches!(x.len(), 2 | 3 | 4 | 7))
                .count()
        })
        .sum()
}

// 0:      1:      2:      3:      4:
//  aaaa    ....    aaaa    aaaa    ....
// b    c  .    c  .    c  .    c  b    c
// b    c  .    c  .    c  .    c  b    c
//  ....    ....    dddd    dddd    dddd
// e    f  .    f  e    .  .    f  .    f
// e    f  .    f  e    .  .    f  .    f
//  gggg    ....    gggg    gggg    ....

//  5:      6:      7:      8:      9:
//  aaaa    aaaa    aaaa    aaaa    aaaa
// b    .  b    .  .    c  b    c  b    c
// b    .  b    .  .    c  b    c  b    c
//  dddd    dddd    ....    dddd    dddd
// .    f  e    f  .    f  e    f  .    f
// .    f  e    f  .    f  e    f  .    f
//  gggg    gggg    ....    gggg    gggg

// 1 => ['c', 'f']
// 7 => ['a', 'c', 'f']
// 4 => ['b', 'c', 'd', 'f']
// 3 => ['a', 'c', 'd', 'f', 'g']
// 2 => ['a', 'c', 'd', 'e', 'g']
// 5 => ['a', 'b', 'd', 'f', 'g']
// 0 => ['a', 'b', 'c', 'e', 'f', 'g']
// 6 => ['a', 'b', 'd', 'e', 'f', 'g']
// 9 => ['a', 'b', 'c', 'd', 'f', 'g']
// 8 => ['a', 'b', 'c', 'd', 'e', 'f', 'g']

pub fn part_2(input: &Data) -> usize {
    input
        .iter()
        .map(|line| {
            let (mut signal_patterns, outputs) = line.clone();
            signal_patterns.sort_by_key(|x| x.len());
            let mut digit_vec = vec![vec![]; 10];
            for signal in signal_patterns {
                let contains = |digit: usize| digit_vec[digit].iter().all(|x| signal.contains(x));
                let difference = |digit: usize| {
                    signal
                        .iter()
                        .filter(|x| !digit_vec[digit].contains(x))
                        .count()
                };
                let digit = match signal.len() {
                    2 => 1,
                    3 => 7,
                    4 => 4,
                    5 => {
                        if contains(7) {
                            3
                        } else if difference(4) == 2 {
                            5
                        } else {
                            2
                        }
                    }
                    6 => {
                        if contains(7) && contains(4) {
                            9
                        } else if contains(5) {
                            6
                        } else {
                            0
                        }
                    }
                    7 => 8,
                    _ => unreachable!(),
                };
                digit_vec[digit] = signal;
            }

            outputs
                .iter()
                .rev()
                .enumerate()
                .fold(0, |acc, (i, output)| {
                    acc + digit_vec.iter().position(|x| x == output).unwrap()
                        * 10_usize.pow(i as u32)
                })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const INPUTS_SMALL: &str = indoc! {"
        acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf
    "};

    const INPUTS_LONG: &str = indoc! {"
        be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
    "};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS_SMALL);
        let result = super::part_1(&input);
        assert_eq!(result, 0);

        let input = super::parse(INPUTS_LONG);
        let result = super::part_1(&input);
        assert_eq!(result, 26);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS_SMALL);
        let result = super::part_2(&input);
        assert_eq!(result, 5353);

        let input = super::parse(INPUTS_LONG);
        let result = super::part_2(&input);
        assert_eq!(result, 61229);
    }
}
