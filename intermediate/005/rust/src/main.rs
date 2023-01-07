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

use std::collections::HashMap;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn find_anagrams(words: Vec<&str>) -> u64 {
    let mut anagrams: HashMap<String, u64> = HashMap::new();
    for word in words {
        let mut sorted_word = word.chars().collect::<Vec<char>>();
        sorted_word.sort();
        let sorted_word = sorted_word.iter().collect::<String>();
        if anagrams.contains_key(&sorted_word) {
            anagrams.insert(sorted_word.clone(), anagrams[&sorted_word] + 1);
        } else {
            anagrams.insert(sorted_word, 0);
        }
    }
    anagrams.values().sum()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        let words = vec![
            "pans", "snap", "pots", "skate", "stake", "word", "test", "cat", "act", "cruel",
            "lucre", "ulcer",
        ];
        assert_eq!(5, find_anagrams(words));
    }
}
