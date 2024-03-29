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
    static ref ELEMENTS: HashMap<String, String> = HashMap::from_iter([
        ("ge".to_string(), "germanium".to_string()),
        ("ni".to_string(), "nickel".to_string()),
        ("u".to_string(),  "uranium".to_string()),
        ("s".to_string(),  "sulfur".to_string()),
    ]);
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn word_in_elements(word: &str) -> String {
    let mut output: String = String::new();
    let mut elements: Vec<String> = Vec::new();
    let mut index = 0;
    let chars = word.chars().collect::<Vec<char>>();
    while index < chars.len() {
        let mut element = String::new();
        for offset in 0..2 {
            if index + offset < chars.len() {
                element.push(chars[index + offset]);
            }
        }
        if let Some(full_element) = ELEMENTS.get(&element) {
            output.push(element.chars().nth(0).unwrap().to_ascii_uppercase());
            if 2 == element.len() {
                output.push(element.chars().nth(1).unwrap().to_ascii_lowercase());
            }
            index += element.len();
            elements.push(full_element.to_string());
        } else {
            element.pop();
            if let Some(full_element) = ELEMENTS.get(&element)  {
                output.push(element.chars().nth(0).unwrap().to_ascii_uppercase());
                elements.push(full_element.to_string());
                index += 1;
            } else {
                return String::new();
            }
        }
    }

    format!("{} ({})", output, elements.join(" "))
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_in_elements() {
        assert_eq!("GeNiUS (germanium nickel uranium sulfur)", word_in_elements("genius"));
        assert_eq!("", word_in_elements("zzz"));
    }
}
