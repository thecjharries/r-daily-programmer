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
use regex::Regex;
use std::collections::BTreeMap;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn compress(input: &str) -> (Vec<String>, String) {
    let mut dictionary: Vec<String> = Vec::new();
    let mut dictionary_map: BTreeMap<String, usize> = BTreeMap::new();
    let mut compressed: Vec<String> = Vec::new();
    lazy_static! {
        static ref ALLOWED_SYMBOLS_PATTERN: Regex = Regex::new(r"([\n\-.,?!;:])").unwrap();
    }
    let input = ALLOWED_SYMBOLS_PATTERN.replace_all(input, " $1 ");
    for token in input.split(" ") {
        println!("token is '{}'", token);
        if "" == token {
            continue;
        } else if 1 == token.len() && '\n' == token.chars().next().unwrap() {
            compressed.push("R".to_string());
        } else if ALLOWED_SYMBOLS_PATTERN.is_match(token) {
            compressed.push(token.to_string());
        } else {
            let lower = token.to_lowercase();
            if !dictionary_map.contains_key(&lower) {
                dictionary_map.insert(lower.clone(), dictionary.len());
                dictionary.push(lower.clone());
            }
            if lower == token {
                compressed.push(dictionary_map[&lower].to_string());
            } else {
                compressed.push(format!("{}^", dictionary_map[&lower]));
            }
        }
    }
    compressed.push("E".to_string());
    (dictionary, compressed.join(" "))
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        // the
        // quick
        // brown
        // fox
        // jumps
        // over
        // lazy
        // dog
        // or
        // did
        // it
        // 0^ 1 2 3 4 5 0 6 7 . R 8^ , 9 10 ? E
        let dictionary = vec![
            "the".to_string(),
            "quick".to_string(),
            "brown".to_string(),
            "fox".to_string(),
            "jumps".to_string(),
            "over".to_string(),
            "lazy".to_string(),
            "dog".to_string(),
            "or".to_string(),
            "did".to_string(),
            "it".to_string(),
        ];
        let compressed = "0^ 1 2 3 4 5 0 6 7 . R 8^ , 9 10 ? E".to_string();
        assert_eq!(
            (dictionary, compressed),
            compress("The quick brown fox jumps over the lazy dog.\nOr, did it?")
        );
    }
}
