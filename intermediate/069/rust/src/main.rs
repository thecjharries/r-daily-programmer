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

#[derive(Debug, PartialEq, Eq)]
struct AdfgvxCipher {
    alphabet: Vec<char>,
    substitution: Vec<char>,
    fraction: HashMap<char, String>,
}

impl AdfgvxCipher {
    pub fn new(alphabet: Vec<char>, substitution: Vec<char>) -> AdfgvxCipher {
        let mut fraction = HashMap::new();
        for (row, row_char) in vec!['A', 'D', 'F', 'G', 'V', 'X'].iter().enumerate() {
            for (column, column_char) in vec!['A', 'D', 'F', 'G', 'V', 'X'].iter().enumerate() {
                let index = row * 5 + column + row;
                let mut fraction_string = String::new();
                fraction_string.push(*row_char);
                fraction_string.push(*column_char);
                fraction.insert(substitution[index], fraction_string);
            }
        }
        AdfgvxCipher {
            alphabet,
            substitution,
            fraction,
        }
    }

    pub fn sanitize(&self, input: &str) -> String {
        input
            .to_uppercase()
            .chars()
            .map(|character| match character {
                'J' => 'I',
                _ => character,
            })
            .filter(|character| self.alphabet.contains(character))
            .collect()
    }

    pub fn fraction_text(&self, input: &str) -> String {
        let sanitized_input = self.sanitize(input);
        let mut output = String::new();
        for character in sanitized_input.chars() {
            output.push_str(self.fraction.get(&character).unwrap());
        }
        output
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
    fn test_adfgvxcipher_new() {
        let cipher = AdfgvxCipher::new(
            "ABCDEFGHIKLMNOPQRSTUVWXYZ0123456789 ".chars().collect(),
            "R3FLMX7KWQ69D4Y5NOZ STV2EH8AP1ICBGU0".chars().collect(),
        );
        assert_eq!(36, cipher.alphabet.len());
        assert_eq!(36, cipher.substitution.len());
        assert_eq!(36, cipher.fraction.len());
        assert_eq!("XX".to_string(), *cipher.fraction.get(&'0').unwrap());
    }

    #[test]
    fn test_adfgvxcipher_sanitize() {
        let input = "Brake me out of jail on the 21st.";
        let cipher = AdfgvxCipher::new(
            "ABCDEFGHIKLMNOPQRSTUVWXYZ0123456789 ".chars().collect(),
            "R3FLMX7KWQ69D4Y5NOZ STV2EH8AP1ICBGU0".chars().collect(),
        );
        assert_eq!("BRAKE ME OUT OF IAIL ON THE 21ST", cipher.sanitize(input));
    }

    #[test]
    fn test_adfgvxcipher_fraction_text() {
        let input = "Brake ";
        let cipher = AdfgvxCipher::new(
            "ABCDEFGHIKLMNOPQRSTUVWXYZ0123456789 ".chars().collect(),
            "R3FLMX7KWQ69D4Y5NOZ STV2EH8AP1ICBGU0".chars().collect(),
        );
        assert_eq!("XFAAVGDDVAGD".to_string(), cipher.fraction_text(input));
    }
}
