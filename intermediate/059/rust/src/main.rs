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
    todo!()
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
    }
}
