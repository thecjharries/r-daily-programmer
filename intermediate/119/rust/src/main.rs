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

#[derive(Debug)]
struct Board {
    board: Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Board {
    fn new(input: String) -> Self {
        let mut start: (usize, usize) = (0, 0);
        let mut end: (usize, usize) = (0, 0);
        let mut board = Vec::new();
        for (row_index, line) in input.split("\n").enumerate() {
            let mut row = Vec::new();
            for (column_index, character) in line.trim().chars().enumerate() {
                match character {
                    'S' => start = (row_index, column_index),
                    'E' => end = (row_index, column_index),
                    _ => (),
                }
                row.push(character);
            }
            board.push(row);
        }
        Self { board, start, end }
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
    fn board_new_generates_a_board() {
        let board = Board::new(
            "S....
        WWWW.
        .....
        .WWWW
        ....E"
                .to_string(),
        );
        assert_eq!(
            vec![
                vec!['S', '.', '.', '.', '.'],
                vec!['W', 'W', 'W', 'W', '.'],
                vec!['.', '.', '.', '.', '.'],
                vec!['.', 'W', 'W', 'W', 'W'],
                vec!['.', '.', '.', '.', 'E'],
            ],
            board.board
        );
        assert_eq!((0, 0), board.start);
        assert_eq!((4, 4), board.end);
    }
}
