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

#[derive(Debug, PartialEq, Eq)]
struct AdfgvxCipher {
    alphabet: Vec<char>,
    substitution: Vec<char>,
}

impl AdfgvxCipher {
    pub fn new(alphabet: Vec<char>, substitution: Vec<char>) -> AdfgvxCipher {
        AdfgvxCipher {
            alphabet,
            substitution,
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
        assert_eq!(
            AdfgvxCipher {
                alphabet: "ABCDEFGHIKLMNOPQRSTUVWXYZ0123456789 ".chars().collect(),
                substitution: "R3FLMX7KWQ69D4Y5NOZ STV2EH8AP1ICBGU0".chars().collect(),
            },
            cipher
        );
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
}
