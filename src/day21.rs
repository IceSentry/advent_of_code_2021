use std::{iter::Cycle, ops::RangeInclusive};

use hashbrown::HashMap;
use serde_scan::scan;

type Data = (usize, usize);
type Die = Cycle<RangeInclusive<usize>>;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Player {
    position: usize,
    score: usize,
}

impl Player {
    fn new(position: usize) -> Self {
        Self { position, score: 0 }
    }

    fn advance_mut(&mut self, amount: usize) {
        self.position = (self.position + amount - 1) % 10 + 1;
        self.score += self.position;
    }

    fn advance(&self, amount: usize) -> Self {
        let position = (self.position + amount - 1) % 10 + 1;
        let score = self.score + position;
        Self { position, score }
    }
}

fn roll(die: &mut Die, roll_count: &mut usize) -> usize {
    let mut roll_sum = 0;
    for _ in 0..3 {
        roll_sum += die.next().expect("Cycle should be infinite");
    }
    *roll_count += 3;
    roll_sum
}

pub fn parse(input: &str) -> Data {
    let players = input
        .lines()
        .flat_map(|l| scan!("Player {} starting position: {}" <- l))
        .collect::<Vec<(usize, usize)>>();

    (players[0].1, players[1].1)
}

pub fn part_1(input: &Data) -> usize {
    let (player_1_pos, player_2_pos) = input;
    let mut player_1 = Player::new(*player_1_pos);
    let mut player_2 = Player::new(*player_2_pos);

    let mut die = (1..=100).cycle();
    let mut roll_count = 0;
    let max_score = 1000;

    loop {
        player_1.advance_mut(roll(&mut die, &mut roll_count));
        if player_1.score >= max_score {
            return player_2.score * roll_count;
        }

        player_2.advance_mut(roll(&mut die, &mut roll_count));
        if player_2.score >= max_score {
            return player_1.score * roll_count;
        }
    }
}

fn compute_win(
    cache: &mut HashMap<(Player, Player), (usize, usize)>,
    player_a: Player,
    player_b: Player,
) -> (usize, usize) {
    let key = (player_a, player_b);
    if cache.contains_key(&key) {
        return cache[&key];
    }

    let mut player_a_wins = 0;
    let mut player_b_wins = 0;
    for a in 1..=3 {
        for b in 1..=3 {
            for c in 1..=3 {
                let player_a = player_a.advance(a + b + c);
                if player_a.score >= 21 {
                    player_a_wins += 1;
                } else {
                    let (player_b_win, player_a_win) = compute_win(cache, player_b, player_a);
                    player_a_wins += player_a_win;
                    player_b_wins += player_b_win;
                }
            }
        }
    }

    let result = (player_a_wins, player_b_wins);
    cache.insert(key, result);
    result
}

pub fn part_2(input: &Data) -> usize {
    let (player_1_pos, player_2_pos) = input;
    let player_1 = Player::new(*player_1_pos);
    let player_2 = Player::new(*player_2_pos);

    let mut cache = HashMap::new();
    let (player_1_wins, player_2_wins) = compute_win(&mut cache, player_1, player_2);
    player_1_wins.max(player_2_wins)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const INPUTS: &str = indoc! {"
        Player 1 starting position: 4
        Player 2 starting position: 8
    "};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let result = super::part_1(&input);
        assert_eq!(result, 739785);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 444356092776315);
    }
}
