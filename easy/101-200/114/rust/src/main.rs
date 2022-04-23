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

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("rad");
}

fn find_adjacent_words(input: &str, dictionary: &Vec<String>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for word in dictionary {
        if word.len() != input.len() {
            continue;
        }
        let mut difference_count = 0;
        for (i, c) in input.chars().enumerate() {
            if c != word.chars().nth(i).unwrap() {
                difference_count += 1;
            }
            if difference_count > 1 {
                break;
            }
        }
        if difference_count == 1 {
            result.push(word.to_string());
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_adjacent_words() {
        let words = BufReader::new(File::open("../selected four-letter words.txt").unwrap())
            .lines()
            .map(|l| l.expect("Could not read line"))
            .collect::<Vec<String>>();
        assert_eq!(
            find_adjacent_words("puma", &words),
            String::from("duma pima puja pula pump puna pupa")
                .split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        );
    }
}
