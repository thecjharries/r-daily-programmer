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

use std::cmp::min;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn condense_sentence(input: &str) -> String {
    let mut exploded = input.split_whitespace().rev().collect::<Vec<&str>>();
    let mut current_word = exploded.pop().unwrap().to_string();
    let mut condensed = Vec::new();
    while 0 < exploded.len() {
        let next = exploded.pop().unwrap().to_string();
        let max_length = min(current_word.len(), next.len());
        let mut max_common = 0;
        for length in 1..=max_length {
            if current_word.ends_with(&next[..length]) {
                max_common = length;
            }
        }
        if 0 < max_common {
            current_word = current_word + &next[max_common..];
        } else {
            condensed.push(current_word);
            current_word = next;
        }
    }
    condensed.push(current_word);
    condensed.join(" ")
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_condense_sentence() {
        assert_eq!(
            "I heard the pastor sing liverses easily.",
            condense_sentence("I heard the pastor sing live verses easily.")
        );
        assert_eq!(
            "Deepisodes of Deep Space Nine came on the televisionly after the news.",
            condense_sentence(
                "Deep episodes of Deep Space Nine came on the television only after the news."
            )
        );
        assert_eq!(
            "Digitalarm clockscarea children.",
            condense_sentence("Digital alarm clocks scare area children.")
        );
    }
}
