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

use lazy_static::lazy_static;

lazy_static! {
    static ref SHIFTED_ALPHABET: String =
        " snvfrghjokl;,mp[wtdyibecuxSNVFRGHJOKL:<MP{WTDYIBECUX".to_string();
    static ref ALPHABET: String =
        " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();
}

fn main() {
    println!("rad");
}

fn decode_shifted(input: &str) -> String {
    let mut output = String::new();
    for c in input.chars() {
        match SHIFTED_ALPHABET.find(c) {
            Some(i) => output.push(ALPHABET.chars().nth(i).unwrap()),
            None => output.push(c),
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_shifted() {
        assert_eq!(decode_shifted("Jr;;p ept;f"), "Hello world".to_string());
        assert_eq!(
            decode_shifted("Lmiyj od ,u jrtp"),
            "Knuth is my hero".to_string()
        );
    }
}
