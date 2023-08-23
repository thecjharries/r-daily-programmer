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

#[derive(Debug, PartialEq)]
struct Alphabet {
    letters: Vec<char>,
}

impl Alphabet {
    fn new(alphabet: &str) -> Self {
        Self {
            letters: alphabet.to_lowercase().chars().collect(),
        }
    }

    fn sort_words(&self, words: Vec<&str>) -> Vec<String> {
        let mut sorted_words = words.to_vec();
        sorted_words.sort_by(|a, b| {
            let a_chars: Vec<char> = a.to_lowercase().chars().collect();
            let b_chars: Vec<char> = b.to_lowercase().chars().collect();
            let mut a_index = 0;
            let mut b_index = 0;
            while a_index < a_chars.len() && b_index < b_chars.len() {
                let a_letter = a_chars[a_index];
                let b_letter = b_chars[b_index];
                if a_letter == b_letter {
                    a_index += 1;
                    b_index += 1;
                } else {
                    let a_letter_index = self.letters.iter().position(|&x| x == a_letter).unwrap();
                    let b_letter_index = self.letters.iter().position(|&x| x == b_letter).unwrap();
                    if a_letter_index < b_letter_index {
                        return std::cmp::Ordering::Less;
                    } else {
                        return std::cmp::Ordering::Greater;
                    }
                }
            }
            if a_index == a_chars.len() && b_index == b_chars.len() {
                std::cmp::Ordering::Equal
            } else if a_index == a_chars.len() {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });
        sorted_words.iter().map(|word| word.to_string()).collect()
    }
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alphabet_can_create_new() {
        let alphabet = Alphabet::new("UVWXYZNOPQRSTHIJKLMABCDEFG");
        assert_eq!(
            Alphabet {
                letters: vec![
                    'u', 'v', 'w', 'x', 'y', 'z', 'n', 'o', 'p', 'q', 'r', 's', 't', 'h', 'i', 'j',
                    'k', 'l', 'm', 'a', 'b', 'c', 'd', 'e', 'f', 'g'
                ]
            },
            alphabet
        );
    }

    #[test]
    fn alphabet_can_sort_based_on_itself() {
        let alphabet = Alphabet::new("UVWXYZNOPQRSTHIJKLMABCDEFG");
        let words = vec![
            "ANTLER", "ANY", "COW", "HILL", "HOW", "HOWEVER", "WHATEVER", "ZONE",
        ];
        let sorted = vec![
            "WHATEVER", "ZONE", "HOW", "HOWEVER", "HILL", "ANY", "ANTLER", "COW",
        ];
        assert_eq!(sorted, alphabet.sort_words(words));
    }
}
