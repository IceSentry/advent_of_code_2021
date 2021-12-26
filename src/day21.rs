use std::{iter::Cycle, ops::RangeInclusive};

use hashbrown::HashMap;
use serde_scan::scan;

type Data = (u16, u16);
type Die = Cycle<RangeInclusive<u16>>;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Player {
    position: u16,
    score: u16,
}

impl Player {
    fn new(position: u16) -> Self {
        Self { position, score: 0 }
    }

    fn advance_mut(&mut self, amount: u16) {
        self.position = (self.position + amount - 1) % 10 + 1;
        self.score += self.position;
    }

    fn advance(&self, amount: u16) -> Self {
        let mut new_player = *self;
        new_player.advance_mut(amount);
        new_player
    }
}

fn roll(die: &mut Die, roll_count: &mut usize) -> u16 {
    let mut roll_sum = 0;
    for _ in 0..3 {
        roll_sum += die.next().expect("Cycle should be infinite");
    }
    *roll_count += 3;
    roll_sum
}

const POSSIBLE_ROLLS: [(u16, usize); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
fn count_wins(
    cache: &mut HashMap<(Player, Player), (usize, usize)>,
    player_a: Player,
    player_b: Player,
) -> (usize, usize) {
    let key = (player_a, player_b);

    if let Some(value) = cache.get(&key) {
        return *value;
    }

    let mut player_a_wins = 0;
    let mut player_b_wins = 0;
    for (value, count) in POSSIBLE_ROLLS {
        let new_player_a = player_a.advance(value);
        if new_player_a.score >= 21 {
            player_a_wins += count;
        } else {
            let (player_b_win, player_a_win) = count_wins(cache, player_b, new_player_a);
            player_a_wins += player_a_win * count;
            player_b_wins += player_b_win * count;
        }
    }

    let result = (player_a_wins, player_b_wins);
    cache.insert(key, result);
    result
}

pub fn parse(input: &str) -> Data {
    let players = input
        .lines()
        .flat_map(|l| scan!("Player {} starting position: {}" <- l))
        .collect::<Vec<(u16, u16)>>();

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
            return player_2.score as usize * roll_count;
        }

        player_2.advance_mut(roll(&mut die, &mut roll_count));
        if player_2.score >= max_score {
            return player_1.score as usize * roll_count;
        }
    }
}

pub fn part_2(input: &Data) -> usize {
    let (player_1_pos, player_2_pos) = input;
    let mut cache = HashMap::new();
    let (player_1_wins, player_2_wins) = count_wins(
        &mut cache,
        Player::new(*player_1_pos),
        Player::new(*player_2_pos),
    );
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
