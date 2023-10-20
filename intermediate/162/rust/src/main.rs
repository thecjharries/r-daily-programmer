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
        let input = r"I would not, could not, in the rain.
Not in the dark. Not on a train.
Not in a car. Not in a tree.
I do not like them, Sam, you see.
Not in a house. Not in a box.
Not with a mouse. Not with a fox.
I will not eat them here or there.
I do not like them anywhere!
";
        // i
        // would
        // not
        // could
        // in
        // the
        // rain
        // dark
        // on
        // a
        // train
        // car
        // tree
        // do
        // like
        // them
        // sam
        // you
        // see
        // house
        // box
        // with
        // mouse
        // fox
        // will
        // eat
        // here
        // or
        // there
        // anywhere
        let dictionary = vec![
            "i".to_string(),
            "would".to_string(),
            "not".to_string(),
            "could".to_string(),
            "in".to_string(),
            "the".to_string(),
            "rain".to_string(),
            "dark".to_string(),
            "on".to_string(),
            "a".to_string(),
            "train".to_string(),
            "car".to_string(),
            "tree".to_string(),
            "do".to_string(),
            "like".to_string(),
            "them".to_string(),
            "sam".to_string(),
            "you".to_string(),
            "see".to_string(),
            "house".to_string(),
            "box".to_string(),
            "with".to_string(),
            "mouse".to_string(),
            "fox".to_string(),
            "will".to_string(),
            "eat".to_string(),
            "here".to_string(),
            "or".to_string(),
            "there".to_string(),
            "anywhere".to_string(),
        ];
        let compressed = "0^ 1 2 , 3 2 , 4 5 6 . R 2^ 4 5 7 . 2^ 8 9 10 . R 2^ 4 9 11 . 2^ 4 9 12 . R 0^ 13 2 14 15 , 16^ , 17 18 . R 2^ 4 9 19 . 2^ 4 9 20 . R 2^ 21 9 22 . 2^ 21 9 23 . R 0^ 24 2 25 15 26 27 28 . R 0^ 13 2 14 15 29 ! R E"
            .to_string();
        assert_eq!((dictionary, compressed), compress(input));
    }
}
