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
use std::collections::HashMap;

lazy_static! {
    static ref LETTER_TO_PBM: HashMap<char, Vec<Vec<u8>>> = HashMap::from_iter([
        (
            'a',
            vec![
                vec![0, 0, 1, 0, 0],
                vec![0, 1, 0, 1, 0],
                vec![1, 0, 0, 0, 1],
                vec![1, 1, 1, 1, 1],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1]
            ]
        ),
        (
            'b',
            vec![
                vec![1, 1, 1, 1, 0],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![1, 1, 1, 1, 0],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 1],
                vec![1, 1, 1, 1, 0]
            ]
        ),
        (
            'c',
            vec![
                vec![0, 1, 1, 1, 0],
                vec![1, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 0],
                vec![1, 0, 0, 0, 0],
                vec![1, 0, 0, 0, 0],
                vec![1, 0, 0, 0, 1],
                vec![0, 1, 1, 1, 0]
            ]
        ),
    ]);
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn create_pbm_input(text: &str) -> Vec<u8> {
    let mut result_lines: Vec<Vec<u8>> = Vec::new();
    for _ in 0..LETTER_TO_PBM.get(&'a').unwrap().len() {
        result_lines.push(Vec::new());
    }
    for letter in text.to_lowercase().chars() {
        match LETTER_TO_PBM.get(&letter) {
            Some(pbm_letter) => {
                for (index, line) in pbm_letter.iter().enumerate() {
                    result_lines[index].extend(line.iter().cloned());
                }
            }
            None => {}
        }
    }
    let mut result: Vec<u8> = Vec::new();
    for line in result_lines {
        result.extend(line);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_pbm_input() {
        assert_eq!("".as_bytes(), create_pbm_input("abc"));
    }
}
