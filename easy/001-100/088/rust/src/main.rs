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

fn vignere_encode(plaintext: &str, key: &str) -> String {
    let plaintext_chars: Vec<char> = plaintext.to_uppercase().chars().collect();
    let key_chars: Vec<char> = key.to_uppercase().chars().collect();
    let mut result: String = String::new();
    for (i, c) in plaintext_chars.iter().enumerate() {
        let key_char = key_chars[i % key_chars.len()];
        let key_char_num = key_char as u8 - 65;
        let c_num = (*c as u8) - 65;
        let result_num = (c_num + key_char_num) % 26;
        result.push((result_num as u8 + 65) as char);
    }
    result
}

fn vignere_decode(ciphertext: &str, key: &str) -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vignere_encode() {
        assert_eq!(vignere_encode("THECAKEISALIE", "GLADOS"), "ZSEFOCKTSDZAK");
    }

    #[test]
    fn test_vignere_decode() {
        assert_eq!(vignere_decode("ZSEFOCKTSDZAK", "GLADOS"), "THECAKEISALIE");
    }
}
