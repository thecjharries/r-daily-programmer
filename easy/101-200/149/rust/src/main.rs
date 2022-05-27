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

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn disemvowel(input: &str) -> (String, String) {
    let mut consonants = String::new();
    let mut vowels = String::new();
    for char in input.to_lowercase().chars() {
        match char {
            'a' | 'e' | 'i' | 'o' | 'u' => vowels.push(char),
            _ => {
                if char.is_alphabetic() {
                    consonants.push(char)
                }
            }
        }
    }
    (consonants, vowels)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disemvowel() {
        assert_eq!(
            (
                "twdrmsndcymblfllffclff".to_string(),
                "ouaaaaoai".to_string()
            ),
            disemvowel("two drums and a cymbal fall off a cliff")
        )
    }
}
