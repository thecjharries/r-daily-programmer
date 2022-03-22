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

use itertools::Itertools;

fn main() {
    println!("rad");
}

fn find_unique_substrings(alphabet: String) -> Vec<String> {
    let mut result = Vec::new();
    for starting_index in 0..alphabet.len() {
        for ending_index in starting_index..alphabet.len() {
            result.push(alphabet[starting_index..=ending_index].to_string());
        }
    }
    result.into_iter().unique().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_unique_substrings() {
        assert_eq!(
            find_unique_substrings("abc".to_string()),
            vec!["a", "ab", "abc", "b", "bc", "c"]
        );
        assert_eq!(
            find_unique_substrings("abcd".to_string()),
            vec!["a", "ab", "abc", "abcd", "b", "bc", "bcd", "c", "cd", "d"]
        );
        assert_eq!(
            find_unique_substrings("hello".to_string()),
            vec![
                "h", "he", "hel", "hell", "hello", "e", "el", "ell", "ello", "l", "ll", "llo",
                "lo", "o"
            ]
        )
    }
}
