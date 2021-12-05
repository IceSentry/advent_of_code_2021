use colored::Colorize;

type Data = (Vec<usize>, Vec<Board>);

#[derive(Clone)]
pub struct Board {
    data: [(usize, bool); 5 * 5],
    is_bingo: bool,
}

impl std::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..5 {
            for x in 0..5 {
                let cell = self.get(x, y);
                if cell.1 {
                    write!(f, "{:>4}", cell.0.to_string().yellow().bold())?;
                } else {
                    write!(f, "{:>4}", cell.0)?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

impl Board {
    fn new(str: &str) -> Self {
        let data = str
            .split('\n')
            .flat_map(|row| {
                row.split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .map(|x| (x, false))
            })
            .collect::<Vec<_>>();
        Self {
            data: data.try_into().unwrap(),
            is_bingo: false,
        }
    }

    fn get(&self, x: usize, y: usize) -> (usize, bool) {
        self.data[x + y * 5]
    }

    fn mark(&mut self, value: usize) -> Option<(usize, usize)> {
        // using iter_mut is apparently much slower, so it's easier to just mutate with the index
        for (i, (cell_value, _)) in self.data.iter().enumerate() {
            if *cell_value == value {
                self.data[i].1 = true;
                return Some((i % 5, i / 5));
            }
        }
        None
    }

    fn check_bingo(&mut self, x: usize, y: usize) -> bool {
        if !self.is_bingo {
            let mut result_x = true;
            for i in 0..5 {
                if !self.get(i, y).1 {
                    result_x = false;
                    break;
                }
            }
            let mut result_y = true;
            for i in 0..5 {
                if !self.get(x, i).1 {
                    result_y = false;
                    break;
                }
            }
            self.is_bingo = result_x || result_y;
        }
        self.is_bingo
    }

    fn get_unmarked_sum(&self) -> usize {
        self.data
            .iter()
            .filter(|(_, marked)| !marked)
            .map(|(cell_value, _)| cell_value)
            .sum()
    }
}

pub fn parse(input: &str) -> Data {
    let mut lines = input.split("\n\n");
    let numbers = lines
        .next()
        .map(|l| {
            l.split(',')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<usize>>()
        })
        .unwrap();
    let boards = lines.map(Board::new).collect::<Vec<_>>();
    (numbers, boards)
}

pub fn part_1(input: &Data) -> usize {
    let (numbers, mut boards) = input.clone();
    for n in numbers {
        for board in boards.iter_mut() {
            if let Some((x, y)) = board.mark(n) {
                if board.check_bingo(x, y) {
                    let sum = board.get_unmarked_sum();
                    // println!("Bingo! {} {}", n, sum);
                    // println!("{:?}", board);
                    return sum * n;
                }
            }
        }
    }
    unreachable!()
}

pub fn part_2(input: &Data) -> usize {
    let (numbers, mut boards) = input.clone();
    for n in numbers {
        for board in boards.iter_mut() {
            if let Some((x, y)) = board.mark(n) {
                board.check_bingo(x, y);
            }
        }
        if boards.len() == 1 && boards[0].is_bingo {
            let sum = boards[0].get_unmarked_sum();
            // println!("Last bingo! {} {}", n, sum);
            // println!("{:?}", boards[0]);
            return sum * n;
        } else {
            boards.retain(|b| !b.is_bingo);
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const INPUTS: &str = indoc! {"
        7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
        8  2 23  4 24
        21  9 14 16  7
        6 10  3 18  5
        1 12 20 15 19

        3 15  0  2 22
        9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6

        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
        2  0 12  3  7
    "};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let result = super::part_1(&input);
        assert_eq!(result, 4512);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 1924);
    }
}
