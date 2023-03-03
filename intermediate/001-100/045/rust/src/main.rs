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

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{BTreeMap, HashMap};

lazy_static! {
    static ref WORD_PATTERN: Regex = Regex::new(r"\w+").unwrap();
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn find_hapaxes(input: &str) -> Vec<String> {
    let mut word_counts: HashMap<String, u32> = HashMap::new();
    for word in WORD_PATTERN.find_iter(input) {
        let word = word.as_str().to_lowercase();
        let count = word_counts.entry(word).or_insert(0);
        *count += 1;
    }
    let mut word_lengths: BTreeMap<u32, Vec<String>> = BTreeMap::new();
    for (word, count) in word_counts {
        if 1 == count {
            let length = word.len() as u32;
            let words = word_lengths.entry(length).or_insert(vec![]);
            words.push(word);
        }
    }
    let max_length = word_lengths.keys().max().unwrap_or(&0);
    word_lengths.get(max_length).unwrap_or(&vec![]).clone()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_hapaxes() {
        assert_eq!(
            vec![String::from("correspondence")],
            find_hapaxes(
                "

        O me! what eyes hath love put in my head,
        Which have no correspondence with true sight,
        Or if they have, where is my judgment fled,
        That censures falsely what they see aright?
    "
            )
        );
    }
}
