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

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn generate_nonogram_clues(
    input: Vec<Vec<char>>,
) -> (Vec<Vec<(char, usize)>>, Vec<Vec<(char, usize)>>) {
    let mut column_clues = Vec::new();
    for column in 0..input[0].len() {
        let mut current_clue = Vec::new();
        let mut current_count = 0;
        let mut current_item = '0';
        for row in 0..input.len() {
            let cell = input[row][column];
            match cell {
                '0' => {
                    if 0 != current_count {
                        current_clue.push((current_item, current_count));
                    }
                    current_count = 0;
                }
                _ => {
                    if current_item != cell {
                        if 0 != current_count {
                            current_clue.push((current_item, current_count));
                        }
                        current_count = 1;
                        current_item = cell;
                    } else {
                        current_count += 1;
                    }
                }
            }
        }
        if 0 != current_count {
            current_clue.push((current_item, current_count));
        }
        column_clues.push(current_clue);
    }
    let mut row_clues = Vec::new();
    for row in input.iter() {
        let mut current_clue = Vec::new();
        let mut current_count = 0;
        let mut current_item = '0';
        for cell in row.iter() {
            match cell {
                '0' => {
                    if 0 != current_count {
                        current_clue.push((current_item, current_count));
                    }
                    current_count = 0;
                }
                _ => {
                    if current_item != *cell {
                        if 0 != current_count {
                            current_clue.push((current_item, current_count));
                        }
                        current_count = 1;
                        current_item = *cell;
                    } else {
                        current_count += 1;
                    }
                }
            }
        }
        if 0 != current_count {
            current_clue.push((current_item, current_count));
        }
        row_clues.push(current_clue);
    }
    (column_clues, row_clues)
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_nonogram_clues() {
        // 0 1 1 1 1 0
        // 1 0 0 1 1 1
        // 1 0 1 1 1 1
        // 1 1 1 1 1 1
        // 0 1 1 1 1 0
        let input = vec![
            vec!['0', '1', '1', '1', '1', '0'],
            vec!['1', '0', '0', '1', '1', '1'],
            vec!['1', '0', '1', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1', '1'],
            vec!['0', '1', '1', '1', '1', '0'],
        ];
        // 3
        // 1 2
        // 1 3
        // 5
        // 5
        // 3
        let column_clues = vec![
            vec![('1', 3)],
            vec![('1', 1), ('1', 2)],
            vec![('1', 1), ('1', 3)],
            vec![('1', 5)],
            vec![('1', 5)],
            vec![('1', 3)],
        ];
        // 4
        // 1 3
        // 1 4
        // 6
        // 4
        let row_clues = vec![
            vec![('1', 4)],
            vec![('1', 1), ('1', 3)],
            vec![('1', 1), ('1', 4)],
            vec![('1', 6)],
            vec![('1', 4)],
        ];
        assert_eq!((column_clues, row_clues), generate_nonogram_clues(input));
        let input = vec![vec!['1', '2'], vec!['2', '1']];
        let column_clues = vec![vec![('1', 1), ('2', 1)], vec![('2', 1), ('1', 1)]];
        let row_clues = vec![vec![('1', 1), ('2', 1)], vec![('2', 1), ('1', 1)]];
        assert_eq!((column_clues, row_clues), generate_nonogram_clues(input));
    }
}
