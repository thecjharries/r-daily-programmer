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

fn remove_consecutive_duplicates(input: &str) -> (String, String) {
    let mut remaining = String::new();
    let mut duplicates = String::new();
    let mut characters = input.chars();
    let mut last_char = characters.next().unwrap();
    remaining.push(last_char);
    for c in characters {
        if c == last_char {
            duplicates.push(c);
        } else {
            remaining.push(c);
            last_char = c;
        }
    }
    (remaining, duplicates)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_consecutive_duplicates() {
        assert_eq!(remove_consecutive_duplicates("ddaaiillyypprrooggrraammeerr"), ("dailyprogramer".to_string(), "dailyprogramer".to_string()));
        assert_eq!(remove_consecutive_duplicates("aabbccddeded"), ("abcdeded".to_string(), "abcd".to_string()));
        assert_eq!(remove_consecutive_duplicates("flabby aapples"), ("flaby aples".to_string(), "bap".to_string()));
        assert_eq!(remove_consecutive_duplicates("aaaa"), ("a".to_string(), "aaa".to_string()));
    }
}
