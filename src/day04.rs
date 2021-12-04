use colored::Colorize;

type Data = (Vec<usize>, Vec<Board>);

#[derive(Clone)]
pub struct Board {
    data: Vec<Vec<(usize, bool)>>,
    is_win: bool,
}

impl std::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..5 {
            for x in 0..5 {
                if self.data[y][x].1 {
                    write!(f, "{:>4}", self.data[y][x].0.to_string().yellow().bold())?;
                } else {
                    write!(f, "{:>4}", self.data[y][x].0)?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

impl Board {
    fn new() -> Self {
        Self {
            data: vec![vec![]; 5],
            is_win: false,
        }
    }

    fn find(&self, value: usize) -> Option<(usize, usize)> {
        for (i, row) in self.data.iter().enumerate() {
            if let Some(index) = row.iter().position(|x| x.0 == value) {
                return Some((index, i));
            }
        }
        None
    }

    fn set(&mut self, position: (usize, usize)) {
        self.data[position.1][position.0].1 = true;
    }

    fn is_win(&mut self, position: (usize, usize)) -> bool {
        if self.is_win {
            return true;
        }

        let mut acc_y = 0;
        let mut acc_x = 0;
        for x in 0..5 {
            if self.data[x][position.0].1 {
                acc_y += 1;
            }
            if self.data[position.1][x].1 {
                acc_x += 1;
            }
        }
        self.is_win = acc_x >= 5 || acc_y >= 5;
        self.is_win
    }

    fn get_unset_sum(&self) -> usize {
        let mut sum = 0;
        for y in 0..5 {
            for x in 0..5 {
                if !self.data[y][x].1 {
                    sum += self.data[y][x].0;
                }
            }
        }
        sum
    }
}

pub fn parse(input: &str) -> Data {
    let mut lines = input.lines();
    let first_line = lines.next();
    let numbers = first_line
        .map(|l| {
            l.split(',')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<usize>>()
        })
        .unwrap();

    let mut row = 0;
    let mut boards = vec![];
    let mut board_index = 0;
    for line in lines {
        if line.trim().is_empty() {
            boards.push(Board::new());
            row = 0;
            board_index += 1;
            continue;
        }
        boards[board_index - 1].data[row] = line
            .split_ascii_whitespace()
            .map(|x| (x.parse().unwrap(), false))
            .collect();
        row += 1;
    }

    (numbers, boards)
}

pub fn part_1(input: &Data) -> usize {
    let (numbers, mut boards) = input.clone();
    for n in numbers {
        for board in boards.iter_mut() {
            if let Some(found_pos) = board.find(n) {
                board.set(found_pos);
                if board.is_win(found_pos) {
                    let sum = board.get_unset_sum();
                    println!("Bingo! {} {}", n, sum);
                    println!("{:?}", board);
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
            if let Some(found_pos) = board.find(n) {
                board.set(found_pos);
                board.is_win(found_pos);
            }
        }
        if boards.len() == 1 && boards.iter().filter(|b| b.is_win).count() == 1 {
            let board = &boards[0];
            let sum = boards[0].get_unset_sum();
            println!("Last bingo! {} {}", n, sum);
            println!("{:?}", board);
            return sum * n;
        } else {
            boards.retain(|b| !b.is_win);
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
