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

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("rad");
}

fn find_words_in_alphabetical_order(list_of_words: Vec<String>) -> Vec<String> {
    let mut words_in_alphabetical_order: Vec<String> = Vec::new();
    for word in list_of_words {
        let mut chars: Vec<char> = word.chars().collect();
        chars.sort();
        let sorted_word = chars.iter().collect::<String>();
        if sorted_word == word {
            words_in_alphabetical_order.push(word);
        }
    }
    words_in_alphabetical_order
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        let lines = BufReader::new(File::open("../enable1.txt").unwrap())
            .lines()
            .map(|l| l.expect("Could not read line"))
            .collect::<Vec<String>>();
        assert_eq!(find_words_in_alphabetical_order(lines).len(), 638);
    }
}
