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
    println!("rad");
}

fn encrypt_caesar(message: String, shift: u8) -> String {
    let mut result = String::new();
    for character in message.chars() {
        if character.is_alphabetic() {
            if character.is_uppercase() {
                result.push(((character as u8 - 'A' as u8 + shift) % 26 + 'A' as u8) as char);
            } else {
                result.push(((character as u8 - 'a' as u8 + shift) % 26 + 'a' as u8) as char);
            }
        } else {
            result.push(character);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_caesar() {
        assert_eq!(encrypt_caesar("rad".to_string(), 1), "sbe".to_string());
        assert_eq!(encrypt_caesar("rad".to_string(), 2), "tcf".to_string());
        assert_eq!(encrypt_caesar("Daily programmer".to_string(), 6), "Jgore vxumxgsskx".to_string());
    }
}
