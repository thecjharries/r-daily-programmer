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

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn encrypt_alphabet_cipher(keyword: &str, plaintext: &str) -> String {
    let mut ciphertext = String::new();
    let mut keyword_index = 0;
    for character in plaintext.chars() {
        let keyword_character = keyword.chars().nth(keyword_index).unwrap();
        let keyword_character_index = ALPHABET.find(keyword_character).unwrap();
        let character_index = ALPHABET.find(character).unwrap();
        let encrypted_character = ALPHABET
            .chars()
            .nth((character_index + keyword_character_index) % 26)
            .unwrap();
        ciphertext.push(encrypted_character);
        keyword_index = (keyword_index + 1) % keyword.len();
    }
    ciphertext
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_alphabet_cipher() {
        assert_eq!(
            "uvrufrsryherugdxjsgozogpjralhvg".to_string(),
            encrypt_alphabet_cipher("bond", "theredfoxtrotsquietlyatmidnight")
        );
        assert_eq!(
            "flrlrkfnbuxfrqrgkefckvsa".to_string(),
            encrypt_alphabet_cipher("train", "murderontheorientexpress")
        );
        assert_eq!(
            "zhvpsyksjqypqiewsgnexdvqkncdwgtixkx".to_string(),
            encrypt_alphabet_cipher("garden", "themolessnuckintothegardenlastnight")
        );
    }
}
