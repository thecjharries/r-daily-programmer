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

use regex::Regex;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn find_longest_word(letters: &str, dictionary: Vec<String>) -> String {
    let mut longest_word = String::new();
    let pattern = Regex::new(format!("^[{}]+$", letters).as_str()).unwrap();
    for word in dictionary {
        if pattern.is_match(word.as_str()) && word.len() > longest_word.len() {
            longest_word = word.to_string();
        }
    }
    longest_word
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(
            "",
            find_longest_word(
                "xxxx",
                vec![
                    "be".to_string(),
                    "bee".to_string(),
                    "ebb".to_string(),
                    "cab".to_string(),
                    "ghost".to_string()
                ]
            )
        );
        assert_eq!(
            "bee",
            find_longest_word(
                "eb",
                vec![
                    "be".to_string(),
                    "bee".to_string(),
                    "ebb".to_string(),
                    "cab".to_string(),
                    "ghost".to_string()
                ]
            )
        );
    }
}
