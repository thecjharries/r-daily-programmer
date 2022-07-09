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
    static ref HASHMAP: HashMap<char, Vec<String>> = HashMap::from_iter([
        (
            '1',
            vec![
                "   ".to_string(),
                "  |".to_string(),
                "  |".to_string(),
                "   ".to_string()
            ]
        ),
        (
            '2',
            vec![
                " _ ".to_string(),
                " _|".to_string(),
                "|_ ".to_string(),
                "   ".to_string()
            ]
        ),
        (
            '3',
            vec![
                " _ ".to_string(),
                " _|".to_string(),
                " _|".to_string(),
                "   ".to_string()
            ]
        ),
        (
            '4',
            vec![
                "   ".to_string(),
                "|_|".to_string(),
                "  |".to_string(),
                "   ".to_string()
            ]
        ),
        (
            '5',
            vec![
                " _ ".to_string(),
                "|_ ".to_string(),
                " _|".to_string(),
                "   ".to_string()
            ]
        ),
        (
            '6',
            vec![
                " _ ".to_string(),
                "|_ ".to_string(),
                "|_|".to_string(),
                "   ".to_string()
            ]
        ),
        (
            '7',
            vec![
                " _ ".to_string(),
                "  |".to_string(),
                "  |".to_string(),
                "   ".to_string()
            ]
        ),
        (
            '8',
            vec![
                " _ ".to_string(),
                "|_|".to_string(),
                "|_|".to_string(),
                "   ".to_string()
            ]
        ),
        (
            '9',
            vec![
                " _ ".to_string(),
                "|_|".to_string(),
                " _|".to_string(),
                "   ".to_string()
            ]
        ),
        (
            '0',
            vec![
                " _ ".to_string(),
                "| |".to_string(),
                "|_|".to_string(),
                "   ".to_string()
            ]
        ),
    ]);
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn bannerize_numbers(input: &str) -> String {
    let mut result_exploded = vec![String::new(), String::new(), String::new(), String::new()];
    for char in input.chars() {
        if let Some(numbers) = HASHMAP.get(&char) {
            for (i, number) in numbers.iter().enumerate() {
                result_exploded[i].push_str(number.clone().as_str());
            }
        }
    }
    result_exploded.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!("\n\n\n", bannerize_numbers("abc"));
        assert_eq!(" _  _  _  _  _  _  _  _  _ \n| || || || || || || || || |\n|_||_||_||_||_||_||_||_||_|\n                           ", bannerize_numbers("000000000"));
    }
}
