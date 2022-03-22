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

fn main() {
    println!("rad");
}

fn find_unique_substrings(alphabet: String) -> Vec<String> {
    if 1 == alphabet.len() {
        return vec![alphabet];
    }
    let mut unique_characters = alphabet.chars().collect::<Vec<char>>();
    unique_characters.reverse();
    let current = unique_characters.pop().unwrap();
    let mut next = find_unique_substrings(unique_characters.iter().collect::<String>());
    let mut result = Vec::new();
    result.push(current.to_string());
    for substring in &next {
        result.push(format!("{}{}", current, substring));
    }
    result.append(&mut next);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_unique_substrings() {
        assert_eq!(
            find_unique_substrings("abc".to_string()),
            vec!["a", "ab", "abc", "ac", "b", "bc", "c"]
        );
        assert_eq!(
            find_unique_substrings("abcd".to_string()),
            vec![
                "a", "ab", "abc", "abcd", "abd", "ac", "acd", "ad", "b", "bc", "bcd", "bd", "c",
                "cd", "d"
            ]
        );
        assert_eq!(
            find_unique_substrings("hello".to_string()),
            vec!["h", "he", "hel", "helo", "e", "el", "elo", "l", "lo", "o"]
        )
    }
}
