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
    println!("Hello, world!");
}

fn encrypt_caesar(input: &str, shift: u32) -> String {
    let mut output = String::new();
    for input_character in input.chars() {
        let mut character = input_character;
        if character.is_alphabetic() {
            let mut character_code = character as u8;
            if character.is_lowercase() {
                character_code = character_code + shift as u8;
                if character_code > 'z' as u8 {
                    character_code = character_code - 26;
                }
            } else {
                character_code = character_code + shift as u8;
                if character_code > 'Z' as u8 {
                    character_code = character_code - 26;
                }
            }
            character = character_code as char;
        }
        output.push(character);
    }
    output
}

fn decrypt_caesar(input: &str, shift: u32) -> String {
    encrypt_caesar(input, 26 - shift)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_caesar() {
        assert_eq!(encrypt_caesar("A", 1), "B");
        assert_eq!(encrypt_caesar("A", 2), "C");
        assert_eq!(encrypt_caesar("A", 3), "D");
        assert_eq!(encrypt_caesar("A", 4), "E");
        assert_eq!(encrypt_caesar("A", 5), "F");
        assert_eq!(encrypt_caesar("A", 6), "G");
        assert_eq!(encrypt_caesar("A", 7), "H");
        assert_eq!(encrypt_caesar("A", 8), "I");
        assert_eq!(encrypt_caesar("A", 9), "J");
        assert_eq!(encrypt_caesar("A", 10), "K");
    }

    #[test]
    fn test_decrypt_caesar() {
        assert_eq!(decrypt_caesar("B", 1), "A");
        assert_eq!(decrypt_caesar("C", 2), "A");
        assert_eq!(decrypt_caesar("D", 3), "A");
        assert_eq!(decrypt_caesar("E", 4), "A");
        assert_eq!(decrypt_caesar("F", 5), "A");
        assert_eq!(decrypt_caesar("G", 6), "A");
        assert_eq!(decrypt_caesar("H", 7), "A");
        assert_eq!(decrypt_caesar("I", 8), "A");
        assert_eq!(decrypt_caesar("J", 9), "A");
        assert_eq!(decrypt_caesar("K", 10), "A");
    }
}
