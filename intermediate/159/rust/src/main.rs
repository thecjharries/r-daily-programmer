// Copyright 2023 CJ Harries
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
enum Winner {
    Human,
    Computer,
    Neither,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Moves {
    Rock,
    Paper,
    Scissors,
    Lizard,
    Spock,
}

impl Moves {
    fn get_winner_as_human(&self, other: &Self) -> Winner {
        match self {
            Self::Rock => match other {
                Self::Rock => Winner::Neither,
                Self::Scissors => Winner::Human,
                Self::Lizard => Winner::Human,
                _ => Winner::Computer,
            },
            Self::Paper => match other {
                Self::Paper => Winner::Neither,
                Self::Rock => Winner::Human,
                Self::Spock => Winner::Human,
                _ => Winner::Computer,
            },
            Self::Scissors => match other {
                Self::Scissors => Winner::Neither,
                Self::Paper => Winner::Human,
                Self::Lizard => Winner::Human,
                _ => Winner::Computer,
            },
            Self::Lizard => match other {
                Self::Lizard => Winner::Neither,
                Self::Paper => Winner::Human,
                Self::Spock => Winner::Human,
                _ => Winner::Computer,
            },
            Self::Spock => match other {
                Self::Spock => Winner::Neither,
                Self::Rock => Winner::Human,
                Self::Scissors => Winner::Human,
                _ => Winner::Computer,
            },
        }
    }
}

struct Games {
    human_score: u32,
    computer_score: u32,
    ties: u32,
    human_plays: BTreeMap<Moves, u32>,
}

impl Default for Games {
    fn default() -> Self {
        let mut human_plays = BTreeMap::new();
        human_plays.insert(Moves::Rock, 0);
        human_plays.insert(Moves::Paper, 0);
        human_plays.insert(Moves::Scissors, 0);
        human_plays.insert(Moves::Lizard, 0);
        human_plays.insert(Moves::Spock, 0);
        Self {
            human_score: 0,
            computer_score: 0,
            ties: 0,
            human_plays,
        }
    }
}

impl Games {
    fn new() -> Self {
        Self::default()
    }

    fn play_round(&mut self, human_move: &Moves, computer_move: &Moves) {
        let winner = human_move.get_winner_as_human(computer_move);
        *self.human_plays.get_mut(human_move).unwrap() += 1;
        match winner {
            Winner::Human => self.human_score += 1,
            Winner::Computer => self.computer_score += 1,
            Winner::Neither => self.ties += 1,
        }
    }
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_win_conditions() {
        assert_eq!(
            Winner::Human,
            Moves::Rock.get_winner_as_human(&Moves::Scissors)
        );
        assert_eq!(
            Winner::Human,
            Moves::Rock.get_winner_as_human(&Moves::Lizard)
        );
        assert_eq!(
            Winner::Computer,
            Moves::Rock.get_winner_as_human(&Moves::Paper)
        );
        assert_eq!(
            Winner::Computer,
            Moves::Rock.get_winner_as_human(&Moves::Spock)
        );
        assert_eq!(
            Winner::Neither,
            Moves::Rock.get_winner_as_human(&Moves::Rock)
        );
        assert_eq!(
            Winner::Human,
            Moves::Paper.get_winner_as_human(&Moves::Rock)
        );
        assert_eq!(
            Winner::Human,
            Moves::Paper.get_winner_as_human(&Moves::Spock)
        );
        assert_eq!(
            Winner::Computer,
            Moves::Paper.get_winner_as_human(&Moves::Scissors)
        );
        assert_eq!(
            Winner::Computer,
            Moves::Paper.get_winner_as_human(&Moves::Lizard)
        );
        assert_eq!(
            Winner::Neither,
            Moves::Paper.get_winner_as_human(&Moves::Paper)
        );
        assert_eq!(
            Winner::Human,
            Moves::Scissors.get_winner_as_human(&Moves::Paper)
        );
        assert_eq!(
            Winner::Human,
            Moves::Scissors.get_winner_as_human(&Moves::Lizard)
        );
        assert_eq!(
            Winner::Computer,
            Moves::Scissors.get_winner_as_human(&Moves::Rock)
        );
        assert_eq!(
            Winner::Computer,
            Moves::Scissors.get_winner_as_human(&Moves::Spock)
        );
        assert_eq!(
            Winner::Neither,
            Moves::Scissors.get_winner_as_human(&Moves::Scissors)
        );
        assert_eq!(
            Winner::Human,
            Moves::Lizard.get_winner_as_human(&Moves::Paper)
        );
        assert_eq!(
            Winner::Human,
            Moves::Lizard.get_winner_as_human(&Moves::Spock)
        );
        assert_eq!(
            Winner::Computer,
            Moves::Lizard.get_winner_as_human(&Moves::Rock)
        );
        assert_eq!(
            Winner::Computer,
            Moves::Lizard.get_winner_as_human(&Moves::Scissors)
        );
        assert_eq!(
            Winner::Neither,
            Moves::Lizard.get_winner_as_human(&Moves::Lizard)
        );
        assert_eq!(
            Winner::Human,
            Moves::Spock.get_winner_as_human(&Moves::Rock)
        );
        assert_eq!(
            Winner::Human,
            Moves::Spock.get_winner_as_human(&Moves::Scissors)
        );
        assert_eq!(
            Winner::Computer,
            Moves::Spock.get_winner_as_human(&Moves::Paper)
        );
        assert_eq!(
            Winner::Computer,
            Moves::Spock.get_winner_as_human(&Moves::Lizard)
        );
        assert_eq!(
            Winner::Neither,
            Moves::Spock.get_winner_as_human(&Moves::Spock)
        );
    }

    #[test]
    fn test_games_new() {
        let games = Games::new();
        assert_eq!(0, games.human_score);
        assert_eq!(0, games.computer_score);
        assert_eq!(0, games.ties);
        assert_eq!(0, *games.human_plays.get(&Moves::Rock).unwrap());
        assert_eq!(0, *games.human_plays.get(&Moves::Paper).unwrap());
        assert_eq!(0, *games.human_plays.get(&Moves::Scissors).unwrap());
        assert_eq!(0, *games.human_plays.get(&Moves::Lizard).unwrap());
        assert_eq!(0, *games.human_plays.get(&Moves::Spock).unwrap());
    }

    #[test]
    fn games_tracks_rounds_played() {
        let mut games = Games::new();
        games.play_round(&Moves::Rock, &Moves::Scissors);
        assert_eq!(1, games.human_score);
        assert_eq!(0, games.computer_score);
        assert_eq!(0, games.ties);
        assert_eq!(1, *games.human_plays.get(&Moves::Rock).unwrap());
        assert_eq!(0, *games.human_plays.get(&Moves::Paper).unwrap());
        assert_eq!(0, *games.human_plays.get(&Moves::Scissors).unwrap());
        assert_eq!(0, *games.human_plays.get(&Moves::Lizard).unwrap());
        assert_eq!(0, *games.human_plays.get(&Moves::Spock).unwrap());
        games.play_round(&Moves::Scissors, &Moves::Rock);
        assert_eq!(1, games.human_score);
        assert_eq!(1, games.computer_score);
        assert_eq!(0, games.ties);
        assert_eq!(1, *games.human_plays.get(&Moves::Rock).unwrap());
        assert_eq!(0, *games.human_plays.get(&Moves::Paper).unwrap());
        assert_eq!(1, *games.human_plays.get(&Moves::Scissors).unwrap());
        assert_eq!(0, *games.human_plays.get(&Moves::Lizard).unwrap());
        assert_eq!(0, *games.human_plays.get(&Moves::Spock).unwrap());
        games.play_round(&Moves::Rock, &Moves::Rock);
        assert_eq!(1, games.human_score);
        assert_eq!(1, games.computer_score);
        assert_eq!(1, games.ties);
        assert_eq!(2, *games.human_plays.get(&Moves::Rock).unwrap());
        assert_eq!(0, *games.human_plays.get(&Moves::Paper).unwrap());
        assert_eq!(1, *games.human_plays.get(&Moves::Scissors).unwrap());
        assert_eq!(0, *games.human_plays.get(&Moves::Lizard).unwrap());
        assert_eq!(0, *games.human_plays.get(&Moves::Spock).unwrap());
    }
}
