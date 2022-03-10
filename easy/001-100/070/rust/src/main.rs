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
use regex::Regex;
use std::collections::HashMap;
use std::fs;

lazy_static! {
    static ref WORD_PATTERN: Regex = Regex::new(r"\w+").unwrap();
}

fn main() {
    println!("rad");
}

fn find_most_common_words(filename: &str, count: usize) -> HashMap<String, i32> {
    let mut word_counts = HashMap::new();
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    for word in WORD_PATTERN.find_iter(&contents) {
        let word = word.as_str().to_string();
        let count = word_counts.entry(word).or_insert(0);
        *count += 1;
    }
    let mut sorted = word_counts.iter().collect::<Vec<_>>();
    sorted.sort_by(|a, b| b.1.cmp(a.1));
    let mut result = HashMap::new();
    for (word, count) in sorted.into_iter().take(count) {
        result.insert(word.to_string(), *count as i32);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_most_common_words() {
        assert_eq!(
            find_most_common_words("../lorem_ipsum.txt", 4),
            HashMap::from([
                ("dolor".to_string(), 6),
                ("dolore".to_string(), 6),
                ("ut".to_string(), 6),
                ("in".to_string(), 9)
            ])
        );
    }
}
