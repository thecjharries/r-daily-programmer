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
    static ref CAPS_MAP: HashMap<char, char> = {
        let mut m = HashMap::new();
        m.insert('a', 'A');
        m.insert('z', 'Z');
        m
    };
    static ref SHIFT_MAP: HashMap<char, char> = {
        let mut m = HashMap::new();
        m.insert('a', 'A');
        m.insert('z', 'Z');
        m.insert('0', ')');
        m
    };
}

fn main() {
    println!("rad");
}

fn key_to_char(key: char, shift: bool, caps: bool) -> char {
    if !shift && !caps {
        key
    } else if shift && !caps {
        *SHIFT_MAP.get(&key).unwrap_or(&key)
    } else if !shift && caps {
        *CAPS_MAP.get(&key).unwrap_or(&key)
    } else {
        let caps_character = *CAPS_MAP.get(&key).unwrap_or(&key);
        let shift_character = *SHIFT_MAP.get(&key).unwrap_or(&key);
        if caps_character == shift_character && caps_character != key {
            key
        } else {
            shift_character
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_to_char() {
        assert_eq!(key_to_char('a', false, false), 'a');
        assert_eq!(key_to_char('a', false, true), 'A');
        assert_eq!(key_to_char('a', true, false), 'A');
        assert_eq!(key_to_char('a', true, true), 'a');
        assert_eq!(key_to_char('0', false, false), '0');
        assert_eq!(key_to_char('0', false, true), '0');
        assert_eq!(key_to_char('0', true, false), ')');
        assert_eq!(key_to_char('0', true, true), ')');
    }
}
