use hashbrown::HashMap;
use serde_scan::scan;

type Pair = (char, char);

type Data = (Vec<char>, HashMap<Pair, (Pair, Pair)>);

pub fn parse(input: &str) -> Data {
    let (template, insertions) = input.split_once("\n\n").unwrap();

    (
        template.chars().collect(),
        insertions
            .lines()
            .flat_map(|l| scan!("{} -> {}" <- l))
            .map(|(pair, output): (String, char)| {
                let mut chars = pair.chars();
                let pair = (chars.next().unwrap(), chars.next().unwrap());
                (pair, ((pair.0, output), (output, pair.1)))
            })
            .collect(),
    )
}

fn solve(input: &Data, iterations: usize) -> usize {
    let (template, rules) = input;

    let mut pairs: HashMap<Pair, usize> = HashMap::new();
    for pair in template.windows(2) {
        *pairs.entry((pair[0], pair[1])).or_insert(0) += 1;
    }
    let mut letters: HashMap<char, usize> = HashMap::new();
    for letter in template {
        *letters.entry(*letter).or_insert(0) += 1;
    }

    for _ in 0..iterations {
        for (pair, count) in pairs.clone() {
            let (pair0, pair1) = rules.get(&pair).expect("pair not found");
            *pairs.entry(*pair0).or_insert(0) += count;
            *pairs.entry(*pair1).or_insert(0) += count;
            *pairs.entry(pair).or_insert(0) -= count;

            // (AB -> C) <=> (AC, CB)
            // A and B have already been counted and we only need to count C once
            *letters.entry(pair0.1).or_insert(0) += count;
        }
    }

    let max = letters.values().max().unwrap();
    let min = letters.values().min().unwrap();
    max - min
}

pub fn part_1(input: &Data) -> usize {
    solve(input, 10)
}

pub fn part_2(input: &Data) -> usize {
    solve(input, 40)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const INPUTS: &str = indoc! {"
        NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C
    "};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let result = super::part_1(&input);
        assert_eq!(result, 1588);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 2188189693529);
    }
}
