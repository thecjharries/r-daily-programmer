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

    fn sort_words(words: Vec<&str>) -> Vec<String> {
        todo!()
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
}
