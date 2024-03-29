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

use std::collections::{BinaryHeap, HashMap};

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

    fn manhattan_distance(first: (usize, usize), second: (usize, usize)) -> usize {
        (first.0 as isize - second.0 as isize).abs() as usize
            + (first.1 as isize - second.1 as isize).abs() as usize
    }

    fn get_neighbors(&self, current: (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        if current.0 > 0 && self.board[current.0 - 1][current.1] != 'W' {
            neighbors.push((current.0 - 1, current.1));
        }
        if current.0 < self.board.len() - 1 && self.board[current.0 + 1][current.1] != 'W' {
            neighbors.push((current.0 + 1, current.1));
        }
        if current.1 > 0 && self.board[current.0][current.1 - 1] != 'W' {
            neighbors.push((current.0, current.1 - 1));
        }
        if current.1 < self.board[current.0].len() - 1
            && self.board[current.0][current.1 + 1] != 'W'
        {
            neighbors.push((current.0, current.1 + 1));
        }
        neighbors
    }

    fn a_star(&self) -> Option<usize> {
        let mut open_set = BinaryHeap::new();
        let mut came_from = HashMap::new();
        let mut g_score = HashMap::new();
        let mut f_score = HashMap::new();
        let mut current = self.start;
        open_set.push((Self::manhattan_distance(current, self.end), current));
        g_score.insert(current, 0);
        f_score.insert(current, Self::manhattan_distance(current, self.end));
        while !open_set.is_empty() {
            current = open_set.pop().unwrap().1;
            if current == self.end {
                return Some(g_score[&current]);
            }
            for neighbor in self.get_neighbors(current) {
                let tentative_g_score = g_score[&current] + 1;
                if !g_score.contains_key(&neighbor) || tentative_g_score < g_score[&neighbor] {
                    came_from.insert(neighbor, current);
                    g_score.insert(neighbor, tentative_g_score);
                    f_score.insert(
                        neighbor,
                        tentative_g_score + Self::manhattan_distance(neighbor, self.end),
                    );
                    open_set.push((f_score[&neighbor], neighbor));
                }
            }
        }
        None
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

    #[test]
    fn manhattan_distance_calculates() {
        assert_eq!(0, Board::manhattan_distance((0, 0), (0, 0)));
        assert_eq!(1, Board::manhattan_distance((0, 0), (0, 1)));
    }

    #[test]
    fn a_star_works() {
        let board = Board::new(
            "S....
        WWWW.
        .....
        .WWWW
        ....E"
                .to_string(),
        );
        assert_eq!(Some(16), board.a_star());
        let board = Board::new(
            "S...W...
            .WW.W.W.
            .W..W.W.
            ......W.
            WWWWWWW.
            E...W...
            WW..WWW.
            ........"
                .to_string(),
        );
        assert_eq!(Some(29), board.a_star());
        let board = Board::new(
            "S...W.W.
            .WW.W.W.
            .W..W.W.
            ......W.
            WWWWWWW.
            E...W...
            WW..WWW.
            ........"
                .to_string(),
        );
        assert_eq!(None, board.a_star());
    }
}
