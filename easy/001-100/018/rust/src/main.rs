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

use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref BAD_CHARACTER_PATTERN: Regex = Regex::new(r"[^0-9a-z]").unwrap();
    static ref LETTER_TO_NUMBER_MAP: HashMap<char, char> = {
        let mut m = HashMap::new();
        m.insert('a', '2');
        m.insert('b', '2');
        m.insert('c', '2');
        m.insert('d', '3');
        m.insert('e', '3');
        m.insert('f', '3');
        m.insert('g', '4');
        m.insert('h', '4');
        m.insert('i', '4');
        m.insert('j', '5');
        m.insert('k', '5');
        m.insert('l', '5');
        m.insert('m', '6');
        m.insert('n', '6');
        m.insert('o', '6');
        m.insert('p', '7');
        m.insert('q', '7');
        m.insert('r', '7');
        m.insert('s', '7');
        m.insert('t', '8');
        m.insert('u', '8');
        m.insert('v', '8');
        m.insert('w', '9');
        m.insert('x', '9');
        m.insert('y', '9');
        m.insert('z', '9');
        m
    };
}

fn main() {
    println!("rad");
}

fn translate_number(input: &str) -> String {
    let mut output = String::new();
    for c in BAD_CHARACTER_PATTERN.replace_all(&input.to_lowercase(), "").chars() {
        match LETTER_TO_NUMBER_MAP.get(&c) {
            Some(n) => output.push(*n),
            None => output.push(c),
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate_number() {
        assert_eq!(translate_number("1-800-COMCAST"), "18002662278".to_string());
    }
}
