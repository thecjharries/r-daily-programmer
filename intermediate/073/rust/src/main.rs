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

fn tidy_matrix(input: &str) -> String {
    let mut transition = Vec::new();
    for line in input.split('\n') {
        transition.push(
            line.chars()
                .filter(|character| character.is_digit(2))
                .collect::<Vec<char>>(),
        );
    }
    let mut kept_row_indices = Vec::new();
    for row in 0..transition.len() {
        if !transition[row].iter().all(|character| *character == '0') {
            kept_row_indices.push(row);
        }
    }
    let mut kept_column_indices = Vec::new();
    for column in 0..transition[0].len() {
        if !transition.iter().all(|row| row[column] == '0') {
            kept_column_indices.push(column);
        }
    }
    let mut output = String::new();
    for row in kept_row_indices {
        for column in kept_column_indices.iter() {
            output.push(transition[row][*column]);
        }
        output.push('\n');
    }
    output.trim().to_string()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tidy_matrix() {
        assert_eq!(
            "01100111
00111101
01100111
01101110
00000011
10100001"
                .to_string(),
            tidy_matrix(
                "0000000000000000
        0000000000000000
        0000011001110000
        0000001111010000
        0000011001110000
        0000011011100000
        0000000000110000
        0000101000010000
        0000000000000000
        0000000000000000
        0000000000000000"
            )
        );
    }
}
