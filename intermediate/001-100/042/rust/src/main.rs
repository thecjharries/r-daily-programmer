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

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref NUMERALS: [char; 7] = ['I', 'V', 'X', 'L', 'C', 'D', 'M'];
    static ref REPLACEMENT_ORDER: [String; 6] = [
        "IIIII".to_string(),
        "VV".to_string(),
        "XXXXX".to_string(),
        "LL".to_string(),
        "CCCCC".to_string(),
        "DD".to_string(),
    ];
    static ref REPLACEMENT_NUMERALS: HashMap<String, String> = HashMap::from_iter(vec![
        ("IIIII".to_string(), "V".to_string()),
        ("VV".to_string(), "X".to_string()),
        ("XXXXX".to_string(), "L".to_string()),
        ("LL".to_string(), "C".to_string()),
        ("CCCCC".to_string(), "D".to_string()),
        ("DD".to_string(), "M".to_string()),
    ]);
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn naive_roman_add(first: &str, second: &str) -> String {
    let mut combined_characters: Vec<char> = first.chars().collect();
    combined_characters.extend(second.chars());
    combined_characters.sort_by(|a, b| {
        NUMERALS
            .iter()
            .position(|&x| x == *b)
            .cmp(&NUMERALS.iter().position(|&x| x == *a))
    });
    let mut output = String::new();
    for character in combined_characters {
        output.push(character);
    }
    for key in REPLACEMENT_ORDER.iter() {
        let value = REPLACEMENT_NUMERALS.get(key).unwrap();
        while output.contains(key) {
            output = output.replace(key, value);
        }
    }
    output
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!("CX".to_string(), naive_roman_add("XXVIII", "LXXXII"));
    }
}
