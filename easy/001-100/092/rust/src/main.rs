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
    static ref CHARACTERS: HashMap<u64, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert(0, vec!["+-+", "| |", "+ +", "| |", "+-+"]);
        m.insert(1, vec!["  +", "  |", "  +", "  |", "  +"]);
        m.insert(2, vec!["+-+", "  |", "+-+", "|  ", "+-+"]);
        m.insert(3, vec!["+-+", "  |", "+-+", "  |", "+-+"]);
        m.insert(4, vec!["+ +", "| |", "+-+", "  |", "  +"]);
        m.insert(5, vec!["+-+", "|  ", "+-+", "  |", "+-+"]);
        m.insert(6, vec!["+-+", "|  ", "+-+", "| |", "+-+"]);
        m.insert(7, vec!["+-+", "  |", "  +", "  |", "  +"]);
        m.insert(8, vec!["+-+", "| |", "+-+", "| |", "+-+"]);
        m.insert(9, vec!["+-+", "| |", "  +", "  |", "+-+"]);
        m
    };
}

fn main() {
    println!("rad");
}

fn print_number(number: u64) -> Vec<String> {
    let mut result = vec!["".to_string(); 5];
    let mut working_number = number;
    while working_number > 0 {
        let digit = working_number % 10;
        let digit_characters = CHARACTERS.get(&digit).unwrap();
        for (index, element) in digit_characters.iter().enumerate() {
            result[index] = format!("{} {}", element, result[index]);
        }
        working_number /= 10;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_number() {
        assert_eq!(
            print_number(5362),
            vec![
                "+-+ +-+ +-+ +-+ ".to_string(),
                "|     | |     | ".to_string(),
                "+-+ +-+ +-+ +-+ ".to_string(),
                "  |   | | | |   ".to_string(),
                "+-+ +-+ +-+ +-+ ".to_string()
            ]
        );
    }
}
