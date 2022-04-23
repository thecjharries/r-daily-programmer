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

fn find_bonus_1(words: &Vec<String>) -> String {
    for word in words {
        if 33 == find_adjacent_words(word, words).len() {
            return word.to_string();
        }
    }
    String::new()
}

fn find_bonus_2(words: &Vec<String>) -> usize {
    let mut found_words: Vec<String> = find_adjacent_words("best", words);
    for _ in 1..3 {
        let mut new_found_words: Vec<String> = Vec::new();
        for word in found_words {
            new_found_words.append(&mut find_adjacent_words(&word, words));
        }
        new_found_words.sort();
        new_found_words.dedup();
        found_words = new_found_words;
    }
    found_words.len()
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
        assert_eq!(
            find_adjacent_words("best", &words),
            vec![
                "bast", "beat", "beet", "belt", "bent", "bust", "gest", "hest", "jest", "lest",
                "nest", "pest", "rest", "test", "vest", "west", "zest"
            ]
        );
    }

    #[test]
    fn test_find_bonus_1() {
        let words = BufReader::new(File::open("../selected four-letter words.txt").unwrap())
            .lines()
            .map(|l| l.expect("Could not read line"))
            .collect::<Vec<String>>();
        assert_eq!(find_bonus_1(&words), "care");
    }

    #[test]
    fn test_bonus_2() {
        let words = BufReader::new(File::open("../selected four-letter words.txt").unwrap())
            .lines()
            .map(|l| l.expect("Could not read line"))
            .collect::<Vec<String>>();
        assert_eq!(find_bonus_2(&words), 575);
    }
}
