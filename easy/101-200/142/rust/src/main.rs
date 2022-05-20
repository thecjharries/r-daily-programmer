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

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn compute_falling_sand(input: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut output = input.clone();
    for row_index in 0..output.len() - 1 {
        for column_index in 0..output[row_index].len() {
            if output[row_index][column_index] == '.' {
                if output[row_index + 1][column_index] == ' ' {
                    output[row_index][column_index] = ' ';
                    output[row_index + 1][column_index] = '.';
                }
            }
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        let input: Vec<Vec<char>> = vec![
            vec!['.', '.', '.', '.', '.'],
            vec![' ', ' ', '#', ' ', ' '],
            vec!['#', ' ', ' ', ' ', ' '],
            vec![' ', ' ', ' ', ' ', ' '],
            vec![' ', ' ', ' ', ' ', '.'],
        ];
        let output: Vec<Vec<char>> = vec![
            vec![' ', ' ', '.', ' ', ' '],
            vec!['.', ' ', '#', ' ', ' '],
            vec!['#', ' ', ' ', ' ', ' '],
            vec![' ', ' ', ' ', ' ', '.'],
            vec![' ', '.', ' ', '.', '.'],
        ];
        assert_eq!(compute_falling_sand(input), output);
    }
}
