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
    transposition: Vec<char>,
    fraction: HashMap<char, String>,
}

impl AdfgvxCipher {
    pub fn new(
        alphabet: Vec<char>,
        substitution: Vec<char>,
        transposition: Vec<char>,
    ) -> AdfgvxCipher {
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
            transposition,
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

    pub fn encrypt(&self, input: &str) -> String {
        let mut output = String::new();
        let fraction_chars = self.fraction_text(input).chars().collect::<Vec<char>>();
        let mut matrix = Vec::new();
        let mut row = 0;
        while row < fraction_chars.len() / self.transposition.len() {
            matrix.push(Vec::new());
            for column in 0..self.transposition.len() {
                let index = (row * self.transposition.len() + column + row) % fraction_chars.len();
                matrix[row].push(fraction_chars[index]);
            }
            row += 1;
        }
        let mut sorted_transposition = self
            .transposition
            .iter()
            .zip(0..self.transposition.len())
            .collect::<Vec<(&char, usize)>>();
        sorted_transposition.sort_by(|a, b| a.0.cmp(&b.0));
        for (_, index) in sorted_transposition {
            for row in 0..matrix.len() {
                output.push(matrix[row][index]);
            }
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
            "PROGRAMMER".chars().collect(),
        );
        assert_eq!(36, cipher.alphabet.len());
        assert_eq!(36, cipher.substitution.len());
        assert_eq!(
            "PROGRAMMER".to_string(),
            cipher.transposition.iter().collect::<String>()
        );
        assert_eq!(36, cipher.fraction.len());
        assert_eq!("XX".to_string(), *cipher.fraction.get(&'0').unwrap());
    }

    #[test]
    fn test_adfgvxcipher_sanitize() {
        let input = "Brake me out of jail on the 21st.";
        let cipher = AdfgvxCipher::new(
            "ABCDEFGHIKLMNOPQRSTUVWXYZ0123456789 ".chars().collect(),
            "R3FLMX7KWQ69D4Y5NOZ STV2EH8AP1ICBGU0".chars().collect(),
            "PROGRAMMER".chars().collect(),
        );
        assert_eq!("BRAKE ME OUT OF IAIL ON THE 21ST", cipher.sanitize(input));
    }

    #[test]
    fn test_adfgvxcipher_fraction_text() {
        let input = "Brake ";
        let cipher = AdfgvxCipher::new(
            "ABCDEFGHIKLMNOPQRSTUVWXYZ0123456789 ".chars().collect(),
            "R3FLMX7KWQ69D4Y5NOZ STV2EH8AP1ICBGU0".chars().collect(),
            "PROGRAMMER".chars().collect(),
        );
        assert_eq!("XFAAVGDDVAGD".to_string(), cipher.fraction_text(input));
    }

    #[test]
    fn test_adfgvxcipher_encrypt() {
        let input = "Brake";
        let cipher = AdfgvxCipher::new(
            "ABCDEFGHIKLMNOPQRSTUVWXYZ0123456789 ".chars().collect(),
            "R3FLMX7KWQ69D4Y5NOZ STV2EH8AP1ICBGU0".chars().collect(),
            "PROGRAMMER".chars().collect(),
        );
        assert_eq!("GVADDAXFVA".to_string(), cipher.encrypt(input));
    }
}
