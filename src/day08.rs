type Data = Vec<(Vec<Vec<char>>, Vec<Vec<char>>)>;

pub fn parse(input: &str) -> Data {
    input
        .lines()
        .map(|line| line.split_once('|').unwrap())
        .map(|(l, r)| {
            (
                l.trim().split(' ').map(|s| s.chars().collect()).collect(),
                r.trim().split(' ').map(|s| s.chars().collect()).collect(),
            )
        })
        .collect()
}

pub fn part_1(input: &Data) -> usize {
    let mut sum = 0;
    for line in input {
        let (_signal_patterns, outputs) = line;
        for output in outputs {
            match output.len() {
                2 | 4 | 3 | 7 => sum += 1,
                _ => (),
            }
        }
    }
    sum
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
    let mut sum = 0;
    for line in input {
        let (mut signal_patterns, outputs) = line.clone();
        signal_patterns.sort_by_key(|x| x.len());
        let mut digit_vec = vec![vec![]; 10];
        for signal in signal_patterns {
            let contains = |digit: usize| digit_vec[digit].iter().all(|x| signal.contains(x));
            let remove = |digit: usize| {
                signal
                    .iter()
                    .filter(|x| !digit_vec[digit].contains(x))
                    .count()
            };
            if signal.len() == 2 {
                digit_vec[1] = signal;
            } else if signal.len() == 3 {
                digit_vec[7] = signal;
            } else if signal.len() == 4 {
                digit_vec[4] = signal;
            } else if signal.len() == 5 {
                if contains(7) {
                    digit_vec[3] = signal;
                } else if remove(4) == 2 {
                    digit_vec[5] = signal;
                } else {
                    digit_vec[2] = signal;
                }
            } else if signal.len() == 6 {
                if contains(3) && contains(5) {
                    digit_vec[9] = signal;
                } else if contains(5) {
                    digit_vec[6] = signal;
                } else {
                    digit_vec[0] = signal;
                }
            } else if signal.len() == 7 {
                digit_vec[8] = signal;
            }
        }

        let mut out = 0;
        for (i, o) in outputs.iter().rev().enumerate() {
            for (digit, chars) in digit_vec.iter().enumerate() {
                if o.len() == chars.len() && o.iter().all(|c| chars.contains(c)) {
                    out += digit * 10_usize.pow(i as u32);
                }
            }
        }

        // println!("{:?}", out);
        sum += out;
    }
    sum
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
        println!("\nshort\n");

        let input = super::parse(INPUTS_SMALL);
        let result = super::part_1(&input);
        assert_eq!(result, 0);

        println!("\nlong\n");

        let input = super::parse(INPUTS_LONG);
        let result = super::part_1(&input);
        assert_eq!(result, 26);
    }

    #[test]
    pub fn part_2() {
        println!("\nshort\n");

        let input = super::parse(INPUTS_SMALL);
        let result = super::part_2(&input);
        assert_eq!(result, 5353);

        println!("\nlong\n");

        let input = super::parse(INPUTS_LONG);
        let result = super::part_2(&input);
        assert_eq!(result, 61229);
    }
}
