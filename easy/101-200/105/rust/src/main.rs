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

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("rad");
}

fn unscramble_words(
    scrambled_words: Vec<&str>,
    word_list: Vec<String>,
) -> HashMap<String, Vec<String>> {
    let sorted_scrambled = scrambled_words
        .iter()
        .map(|s| {
            let mut sorted = s.chars().collect::<Vec<char>>();
            sorted.sort();
            sorted
        })
        .collect::<Vec<Vec<char>>>();
    let mut unscrambled_words: HashMap<String, Vec<String>> = HashMap::new();
    for word in word_list {
        let mut sorted_word = word.chars().collect::<Vec<char>>();
        sorted_word.sort();
        for (index, scrambled) in sorted_scrambled.iter().enumerate() {
            if sorted_word == *scrambled {
                let key = scrambled_words[index].to_string();
                if unscrambled_words.contains_key(&key) {
                    unscrambled_words
                        .get_mut(&key)
                        .unwrap()
                        .push(word.to_string());
                } else {
                    unscrambled_words.insert(key, vec![word.to_string()]);
                }
            }
        }
    }
    unscrambled_words
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        let words = BufReader::new(File::open("../enable1.txt").unwrap())
            .lines()
            .map(|l| l.expect("Could not read line"))
            .collect::<Vec<String>>();
        let scrambled_words = vec!["tac", "eeb", "ogd"];
        let expected: HashMap<String, Vec<String>> = HashMap::from_iter(vec![
            (
                "tac".to_string(),
                vec!["act".to_string(), "cat".to_string()],
            ),
            ("eeb".to_string(), vec!["bee".to_string()]),
            (
                "ogd".to_string(),
                vec!["dog".to_string(), "god".to_string()],
            ),
        ]);
        assert_eq!(unscramble_words(scrambled_words, words), expected);
    }
}
