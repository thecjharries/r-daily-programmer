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

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn build_score(input: &str) -> HashMap<char, i32> {
    let mut scores: HashMap<char, i32> = HashMap::new();
    for character in input.chars() {
        let mut score = 0;
        if character.is_uppercase() {
            score -= 1;
        } else {
            score += 1;
        }
        scores.entry(character.to_ascii_lowercase()).and_modify(|value| *value += score).or_insert(score);
    }
    scores
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(
            build_score("abcde"),
            [('a', 1), ('b', 1), ('c', 1), ('d', 1), ('e', 1)]
                .iter()
                .cloned()
                .collect()
        );
        assert_eq!(
            build_score("dbbaCEDbdAacCEAadcB"),
            [('a', 1), ('b', 2), ('c', 0), ('d', 2), ('e', -2)]
                .iter()
                .cloned()
                .collect()
        );
    }
}
