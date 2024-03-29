// Copyright 2022 CJ Harries
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

use lazy_static::lazy_static;

lazy_static! {
    static ref SOLUTIONS: Vec<Vec<u8>> = vec![
        vec![0, 1, 2],
        vec![3, 4, 5],
        vec![6, 7, 8],
        vec![0, 3, 6],
        vec![1, 4, 7],
        vec![2, 5, 8],
        vec![0, 4, 8],
        vec![2, 4, 6],
    ];
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn find_winning_move(board: Vec<char>, piece: char) -> Result<u8, String> {
    for possible_solution in SOLUTIONS.iter() {
        let mut found_count = 0;
        let mut solution_index: u8 = 0;
        for index in possible_solution.iter() {
            if board[*index as usize] == piece {
                found_count += 1;
            } else {
                solution_index = *index
            }
        }
        if 2 == found_count {
            return Ok(solution_index);
        }
    }
    Err("No winning move".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_winning_move() {
        let board: Vec<char> = vec!['-', 'O', 'O', 'X', 'X', '-', '-', '-', '-'];
        assert_eq!(0, find_winning_move(board.clone(), 'O').unwrap());
        assert_eq!(5, find_winning_move(board.clone(), 'X').unwrap());
        assert!(find_winning_move(board.clone(), 'Y').is_err());
    }
}
