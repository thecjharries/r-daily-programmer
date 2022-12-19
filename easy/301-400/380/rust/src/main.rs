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
use std::collections::HashMap;

lazy_static! {
    static ref ROMAN_TO_MORSE: HashMap<char, String> = HashMap::from([
        ('0', "-----".to_string()),
        ('1', ".----".to_string()),
        ('2', "..---".to_string()),
        ('3', "...--".to_string()),
        ('4', "....-".to_string()),
        ('5', ".....".to_string()),
        ('6', "-....".to_string()),
        ('7', "--...".to_string()),
        ('8', "---..".to_string()),
        ('9', "----.".to_string()),
        ('a', ".-".to_string()),
        ('b', "-...".to_string()),
        ('c', "-.-.".to_string()),
        ('d', "-..".to_string()),
        ('e', ".".to_string()),
        ('f', "..-.".to_string()),
        ('g', "--.".to_string()),
        ('h', "....".to_string()),
        ('i', "..".to_string()),
        ('j', ".---".to_string()),
        ('k', "-.-".to_string()),
        ('l', ".-..".to_string()),
        ('m', "--".to_string()),
        ('n', "-.".to_string()),
        ('o', "---".to_string()),
        ('p', ".--.".to_string()),
        ('q', "--.-".to_string()),
        ('r', ".-.".to_string()),
        ('s', "...".to_string()),
        ('t', "-".to_string()),
        ('u', "..-".to_string()),
        ('v', "...-".to_string()),
        ('w', ".--".to_string()),
        ('x', "-..-".to_string()),
        ('y', "-.--".to_string()),
        ('z', "--..".to_string()),
        ('.', ".-.-.-".to_string()),
        (',', "--..--".to_string()),
        ('?', "..--..".to_string()),
        ('!', "-.-.--".to_string()),
        ('-', "-....-".to_string()),
        ('/', "-..-.".to_string()),
        ('@', ".--.-.".to_string()),
        ('(', "-.--.".to_string()),
        (')', "-.--.-".to_string()),
        (' ', "/".to_string()),
    ]);
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn smorse(input: &str) -> String {
    let mut output = String::new();
    for character in input.chars() {
        output.push_str(ROMAN_TO_MORSE.get(&character).unwrap());
    }
    output
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!("...---...".to_string(), smorse("sos"));
        assert_eq!("-...-...-..-.--".to_string(), smorse("daily"));
        assert_eq!(
            ".--..-.-----..-..-----..-.".to_string(),
            smorse("programmer")
        );
        assert_eq!("-.....-...".to_string(), smorse("bits"));
        assert_eq!("-.....-...".to_string(), smorse("three"));
    }
}
