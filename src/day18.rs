use itertools::Itertools;

type Data = Vec<SnaifishNumber>;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct SnaifishNumber {
    values: Vec<usize>,
    depths: Vec<usize>,
}

impl SnaifishNumber {
    fn parse(str: &str) -> Self {
        let mut values = vec![];
        let mut depths = vec![];

        let mut depth = 0;
        let mut number = String::new();
        let mut chars = str.chars();
        loop {
            match chars.next() {
                Some('[') => depth += 1,
                Some(c) if c.is_digit(10) => {
                    number = format!("{}{}", number, c);
                }
                c => {
                    if !number.is_empty() {
                        values.push(
                            number
                                .parse()
                                .unwrap_or_else(|_| panic!("failed to parse number {}", number)),
                        );
                        number = String::new();
                        depths.push(depth);
                    }

                    if let Some(']') = c {
                        depth -= 1;
                    }

                    if c.is_none() {
                        return Self { values, depths };
                    }
                }
            }
        }
    }

    fn add(self, other: SnaifishNumber) -> SnaifishNumber {
        let mut values = self.values;
        values.extend(other.values);
        let mut depths = self.depths;
        depths.extend(other.depths);
        for d in depths.iter_mut() {
            *d += 1;
        }
        SnaifishNumber { values, depths }
    }

    fn reduce(&mut self) {
        loop {
            if !self.explode() && !self.split() {
                break;
            }
        }
    }

    fn explode(&mut self) -> bool {
        for i in 0..self.depths.len() {
            if self.depths[i] <= 4 {
                continue;
            }

            // the pair's left value is added to the first regular number to the left
            if i > 0 {
                self.values[i - 1] += self.values[i];
            }
            // the pair's right value is added to the first regular number to the right
            if self.values.len() > i + 2 {
                self.values[i + 2] += self.values[i + 1];
            }
            // the entire exploding pair is replaced with the regular number 0
            self.values[i] = 0;
            self.values.remove(i + 1);
            self.depths[i] -= 1;
            self.depths.remove(i + 1);
            return true;
        }
        false
    }

    fn split(&mut self) -> bool {
        for i in 0..self.depths.len() {
            if self.values[i] < 10 {
                continue;
            }

            let value = self.values[i] as f32;
            self.values.remove(i);
            self.values.insert(i, (value / 2.).floor() as usize);
            self.values.insert(i + 1, (value / 2.).ceil() as usize);

            self.depths[i] += 1;
            self.depths.insert(i + 1, self.depths[i]);
            return true;
        }
        false
    }

    fn magnitude(&mut self) -> usize {
        let mut values = self.values.clone();
        let mut depths = self.depths.clone();
        while values.len() > 1 {
            for i in 0..values.len() - 1 {
                if depths[i] == depths[i + 1] {
                    values[i] = 3 * values[i] + 2 * values[i + 1];
                    if depths[i] > 0 {
                        depths[i] -= 1;
                    }

                    values.remove(i + 1);
                    depths.remove(i + 1);
                    break;
                }
            }
        }
        values[0]
    }
}

pub fn parse(input: &str) -> Data {
    input.lines().map(SnaifishNumber::parse).collect()
}

pub fn part_1(input: &Data) -> usize {
    let mut result = input.first().unwrap().clone();
    for tree in input.iter().cloned().skip(1) {
        result = result.add(tree);
        result.reduce();
    }
    result.magnitude()
}

pub fn part_2(input: &Data) -> usize {
    let mut max = 0;
    for a in input.iter() {
        for b in input.iter() {
            if a != b {
                let mut result = a.clone().add(b.clone());
                result.reduce();
                max = max.max(result.magnitude())
            }
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::SnaifishNumber;

    const INPUTS: &str = indoc! {"
        [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
        [[[5,[2,8]],4],[5,[[9,9],0]]]
        [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
        [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
        [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
        [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
        [[[[5,4],[7,7]],8],[[8,3],8]]
        [[9,3],[[9,9],[6,[4,9]]]]
        [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
        [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
    "};

    #[test]
    pub fn parse() {
        let assert_parse = |input, expected_values: Vec<usize>, expected_depths: Vec<usize>| {
            let tree = SnaifishNumber::parse(input);
            assert_eq!(tree.values, expected_values);
            assert_eq!(tree.depths, expected_depths);
        };

        assert_parse("10", vec![10], vec![0]);
        assert_parse("[0,0]", vec![0, 0], vec![1, 1]);
        assert_parse("[[1,9],[8,5]]", vec![1, 9, 8, 5], vec![2, 2, 2, 2]);
        assert_parse(
            "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]",
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![4, 4, 4, 4, 4, 4, 4, 4, 1],
        );
        assert_parse(
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
            vec![3, 2, 1, 7, 3, 6, 5, 4, 3, 2],
            vec![2, 3, 4, 5, 5, 2, 3, 4, 5, 5],
        );
    }

    #[test]
    pub fn add() {
        let assert_add = |a, b, expected| {
            let a = SnaifishNumber::parse(a);
            let b = SnaifishNumber::parse(b);
            let result = a.add(b);
            assert_eq!(result, SnaifishNumber::parse(expected));
        };

        assert_add("[1,2]", "[[3,4],5]", "[[1,2],[[3,4],5]]");
        assert_add("[1,1]", "[2,2]", "[[1,1],[2,2]]");
        assert_add("[[1,1],[2.2]]", "[3,3]", "[[[1,1],[2,2]],[3,3]]");
        assert_add(
            "[[[1,1],[2,2]],[3,3]]",
            "[4,4]",
            "[[[[1,1],[2,2]],[3,3]],[4,4]]",
        );
        assert_add(
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
            "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
            "[[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]",
        );
    }

    #[test]
    pub fn explode() {
        let assert_explode = |input, expected| {
            let mut input = SnaifishNumber::parse(input);
            input.explode();
            assert_eq!(input, SnaifishNumber::parse(expected));
        };

        assert_explode("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]");
        assert_explode("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]");
        assert_explode("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]");
        assert_explode(
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
        );
        assert_explode(
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
        );
    }

    #[test]
    pub fn split() {
        let assert_split = |input, expected| {
            let mut input = SnaifishNumber::parse(input);
            input.split();
            assert_eq!(input, SnaifishNumber::parse(expected));
        };
        assert_split("[10,0]", "[[5,5],0]");
        assert_split("11", "[5,6]");
        assert_split("12", "[6,6]");
        assert_split(
            "[[[[0,7],4],[15,[0,13]]],[1,1]]",
            "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
        );
        assert_split(
            "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
            "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]",
        );
    }

    #[test]
    pub fn addition() {
        let assert_add = |input, expected| {
            let input = super::parse(input);
            let mut result = input.first().unwrap().clone();
            for tree in input.iter().cloned().skip(1) {
                result = result.add(tree);
                result.reduce();
            }
            assert_eq!(result, SnaifishNumber::parse(expected));
        };

        assert_add(
            indoc! {"
                [1,1]
                [2,2]
                [3,3]
                [4,4]
            "},
            "[[[[1,1],[2,2]],[3,3]],[4,4]]",
        );
        assert_add(
            indoc! {"
                [1,1]
                [2,2]
                [3,3]
                [4,4]
                [5,5]
            "},
            "[[[[3,0],[5,3]],[4,4]],[5,5]]",
        );
        assert_add(
            indoc! {"
                [1,1]
                [2,2]
                [3,3]
                [4,4]
                [5,5]
                [6,6]
            "},
            "[[[[5,0],[7,4]],[5,5]],[6,6]]",
        );
        assert_add(
            indoc! {"
                [[[[4,3],4],4],[7,[[8,4],9]]]
                [1,1]
            "},
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
        );
        assert_add(
            indoc! {"
                [[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
                [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
            "},
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
        );
        assert_add(
            indoc! {"
                [[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]
                [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
            "},
            "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]",
        );
        assert_add(
            indoc! {"
                [[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
                [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
                [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
                [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
                [7,[5,[[3,8],[1,4]]]]
                [[2,[2,2]],[8,[8,1]]]
                [2,9]
                [1,[[[9,3],9],[[9,0],[0,7]]]]
                [[[5,[7,4]],7],1]
                [[[[4,2],2],6],[8,7]]
            "},
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
        );
        assert_add(
            INPUTS,
            "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]",
        );
    }

    #[test]
    pub fn magnitude() {
        let assert_magnitude = |input, expected| {
            let mut input = SnaifishNumber::parse(input);
            assert_eq!(input.magnitude(), expected);
        };
        assert_magnitude("[[1,2],[[3,4],5]]", 143);
        assert_magnitude("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384);
        assert_magnitude("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445);
        assert_magnitude("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791);
        assert_magnitude(
            "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]",
            4140,
        );
    }

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let result = super::part_1(&input);
        assert_eq!(result, 4140);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 3993);
    }
}
