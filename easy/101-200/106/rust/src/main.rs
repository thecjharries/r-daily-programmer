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
use std::fs::read_to_string;

lazy_static! {
    static ref PROMPT_PATTERN: Regex =
        Regex::new(r#"[".,:;!?()\[\]{}]|[^".,:;!?()\[\]{}\s]+"#).unwrap();
}

fn main() {
    println!("rad");
}

fn determine_top_ten_words(text: String) -> Vec<(String, u32)> {
    let words = PROMPT_PATTERN.find_iter(text.as_str());
    let mut word_counts = std::collections::HashMap::new();
    for word in words {
        let word = word.as_str();
        let count = word_counts.entry(word).or_insert(0);
        *count += 1;
    }
    let mut top_ten = word_counts
        .iter()
        .map(|(word, count)| (word.to_string(), *count))
        .collect::<Vec<_>>();
    top_ten.sort_by(|a, b| b.1.cmp(&a.1));
    top_ten.truncate(10);
    top_ten
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_top_ten_words() {
        let text = read_to_string("../pg5200.txt").unwrap();
        assert_eq!(
            determine_top_ten_words(text),
            vec!["the", "and", "to", "of", "a", "in", "that", "have", "it", "for"]
        );
    }
}
