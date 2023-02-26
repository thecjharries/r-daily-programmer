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

#[derive(Debug, PartialEq, Clone, Copy)]
enum GameState {
    Playing,
    Win,
    Lose,
    Draw,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
enum Move {
    First = b'X',
    Second = b'O',
}

impl Into<char> for Move {
    fn into(self) -> char {
        self as u8 as char
    }
}

impl From<char> for Move {
    fn from(character: char) -> Self {
        match character {
            'X' => Move::First,
            'O' => Move::Second,
            _ => panic!("Invalid move"),
        }
    }
}

struct TicTacToe {
    grid: Vec<char>,
    state: GameState,
    player_move_next: bool,
}

impl Default for TicTacToe {
    fn default() -> Self {
        TicTacToe {
            grid: vec![' '; 9],
            state: GameState::Playing,
            player_move_next: true,
        }
    }
}

impl TicTacToe {
    fn turn(&mut self, x: usize, y: usize, character: char) {
        let move_type = Move::from(character);
        self.grid[x + y * 3] = move_type.into();
        self.player_move_next = !self.player_move_next;
        self.check_state();
    }

    fn check_state(&mut self) {
        let winning_combinations = vec![
            vec![0, 1, 2],
            vec![3, 4, 5],
            vec![6, 7, 8],
            vec![0, 3, 6],
            vec![1, 4, 7],
            vec![2, 5, 8],
            vec![0, 4, 8],
            vec![2, 4, 6],
        ];
        for combination in winning_combinations.iter() {
            if self.grid[combination[0]] == self.grid[combination[1]]
                && self.grid[combination[1]] == self.grid[combination[2]]
                && self.grid[combination[0]] != ' '
            {
                self.state = if self.player_move_next {
                    GameState::Lose
                } else {
                    GameState::Win
                };
            }
        }
        if self.grid.iter().all(|&character| character != ' ') {
            self.state = GameState::Draw;
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
    fn test_game_default() {
        let game = TicTacToe::default();
        assert_eq!(vec![' '; 9], game.grid);
        assert_eq!(GameState::Playing, game.state);
        assert_eq!(true, game.player_move_next);
    }

    #[test]
    fn test_move_as_char() {
        assert_eq!('X', Move::First.into());
        assert_eq!('O', Move::Second.into());
    }

    #[test]
    #[should_panic(expected = "Invalid move")]
    fn test_char_as_move() {
        assert_eq!(Move::First, Move::from('X'));
        assert_eq!(Move::Second, Move::from('O'));
        assert!(Move::from('Z') == Move::First);
    }

    #[test]
    fn test_player_win() {
        let mut game = TicTacToe::default();
        game.player_move_next = false;
        game.grid = vec!['X', 'X', 'X', ' ', ' ', ' ', ' ', ' ', ' '];
        game.check_state();
        assert_eq!(GameState::Win, game.state);
    }

    #[test]
    fn test_player_lose() {
        let mut game = TicTacToe::default();
        game.grid = vec!['O', 'O', 'O', ' ', ' ', ' ', ' ', ' ', ' '];
        game.check_state();
        assert_eq!(GameState::Lose, game.state);
    }

    #[test]
    fn test_game_draw() {
        let mut game = TicTacToe::default();
        game.grid = vec!['X', 'O', 'X', 'O', 'X', 'O', 'O', 'X', 'O'];
        game.check_state();
        assert_eq!(GameState::Draw, game.state);
    }

    #[test]
    fn test_game_playing() {
        let mut game = TicTacToe::default();
        game.grid = vec!['X', 'O', 'X', 'O', 'X', 'O', 'O', 'X', ' '];
        game.check_state();
        assert_eq!(GameState::Playing, game.state);
    }

    #[test]
    fn test_game_turn() {
        let mut game = TicTacToe::default();
        game.turn(0, 0, 'X');
        assert_eq!(vec!['X', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], game.grid);
    }
}
