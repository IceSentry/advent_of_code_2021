use hashbrown::HashSet;
use serde_scan::scan;

type Data = (HashSet<(usize, usize)>, Vec<(char, usize)>);

pub fn parse(input: &str) -> Data {
    let (dots, folds) = input.split_once("\n\n").unwrap();

    (
        dots.lines()
            .flat_map(|l| l.split_once(","))
            .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
            .collect(),
        folds
            .lines()
            .flat_map(|l| scan!("fold along {}={}" <- l))
            .collect(),
    )
}

fn fold(dots: &mut HashSet<(usize, usize)>, fold_line: char, fold_value: usize) {
    let folded = dots
        .drain_filter(|(x, y)| match fold_line {
            'y' => y > &fold_value,
            'x' => x > &fold_value,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();

    for (x, y) in folded {
        let dot = match fold_line {
            'y' => (x, fold_value - (y - fold_value)),
            'x' => (fold_value - (x - fold_value), y),
            _ => unreachable!(),
        };
        if !dots.contains(&dot) {
            dots.insert(dot);
        }
    }
}

fn print_output(dots: &HashSet<(usize, usize)>) {
    let letter_height = 6;
    let letter_width = 4;
    let mut message_width = 8 * letter_width;
    message_width += 7; // add spaces between letters

    for y in 0..letter_height {
        for x in 0..message_width {
            if dots.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

pub fn part_1(input: &Data) -> usize {
    let (mut dots, folds) = input.clone();
    let (fold_line, fold_value) = folds[0];

    fold(&mut dots, fold_line, fold_value);

    dots.len()
}

pub fn part_2(input: &Data) -> usize {
    let (mut dots, folds) = input.clone();
    for (fold_line, fold_value) in folds {
        fold(&mut dots, fold_line, fold_value);
    }

    print_output(&dots);

    dots.len()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const INPUTS: &str = indoc! {"
        6,10
        0,14
        9,10
        0,3
        10,4
        4,11
        6,0
        6,12
        4,1
        0,13
        10,12
        3,4
        3,0
        8,4
        1,10
        2,14
        8,10
        9,0

        fold along y=7
        fold along x=5
    "};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let result = super::part_1(&input);
        assert_eq!(result, 17);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 16);
    }
}
