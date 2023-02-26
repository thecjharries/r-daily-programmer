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

enum GameState {
    Playing
    Win,
    Lose,
    Draw,
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
}
