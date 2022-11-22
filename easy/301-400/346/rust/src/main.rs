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
use std::collections::HashMap;

const NUMBERS: [usize; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn find_word_mapping(words: Vec<&str>) -> HashMap<char, usize> {
    let mut unique_characters: Vec<char> = Vec::new();
    for word in words.iter() {
        for character in word.chars() {
            if !unique_characters.contains(&character) {
                unique_characters.push(character);
            }
        }
    }
    let mut character_mapping: HashMap<char, usize> = HashMap::new();
    for combination in NUMBERS.iter().permutations(unique_characters.len()) {
        character_mapping.clear();
        for (index, character) in unique_characters.iter().enumerate() {
            character_mapping.insert(*character, *combination[index]);
        }
        let mut word_starts_with_zero = false;
        for word in words.iter() {
            if let Some(first_character) = word.chars().next() {
                if 0 == character_mapping[&first_character] {
                    word_starts_with_zero = true;
                    break;
                }
            }
        }
        if word_starts_with_zero {
            continue;
        }
        let mut result = 0;
        for (index, character) in words[words.len() - 1].chars().rev().enumerate() {
            result += character_mapping[&character] * 10usize.pow(index as u32);
        }
        let mut sum = 0;
        for word in words.iter().take(words.len() - 1) {
            for (index, character) in word.chars().rev().enumerate() {
                sum += character_mapping[&character] * 10usize.pow(index as u32);
            }
        }
        if result == sum {
            break;
        }
    }
    character_mapping
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_word_mapping() {
        assert_eq!(
            HashMap::from_iter(vec![
                ('D', 7),
                ('E', 5),
                ('M', 1),
                ('N', 6),
                ('O', 0),
                ('R', 8),
                ('S', 9),
                ('Y', 2),
            ]),
            find_word_mapping(vec!["SEND", "MORE", "MONEY"])
        );
    }
}
