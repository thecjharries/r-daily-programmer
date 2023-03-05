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

use itertools::Itertools;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn determine_possible_words(input: &str) -> Vec<String> {
    let mut results = Vec::new();
    let mut index = 0;
    match input.chars().nth(index) {
        Some('(') => {
            let mut characters = Vec::new();
            while input.len() > index {
                index += 1;
                match input.chars().nth(index) {
                    Some(')') => {
                        break;
                    }
                    Some(character) => {
                        characters.push(character);
                    }
                    None => break,
                }
            }
            for character in characters {
                let new_input = format!("{}{}", character, &input[index + 1..]);
                results.append(&mut determine_possible_words(&new_input));
            }
        }
        Some(character) => {
            let sub_results = determine_possible_words(&input[index + 1..]);
            if sub_results.is_empty() {
                results.push(character.to_string());
            } else {
                for sub_result in sub_results {
                    results.push(format!("{}{}", character, sub_result));
                }
            }
        }
        None => (),
    }
    results.into_iter().unique().collect()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(vec!["rad"], determine_possible_words("rad"));
        assert_eq!(vec!["rad", "bad"], determine_possible_words("(rb)ad"));
        assert_eq!(vec!["rad", "bad"], determine_possible_words("(rbb)ad"));
    }
}
