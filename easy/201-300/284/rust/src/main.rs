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

use std::fs::{read_to_string, remove_file, File};
use std::io::prelude::*;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn load_dictionary(path: &str) -> Vec<String> {
    read_to_string(path)
        .expect("Unable to read dictionary")
        .trim()
        .split("\n")
        .map(|word| word.to_string())
        .collect()
}

fn find_swipe_words(letters: &str, min_length: usize, dictionary: &Vec<String>) -> Vec<String> {
    dictionary
        .iter()
        .filter(|word| &word[0..1] == &letters[0..1])
        .filter(|word| word.len() >= min_length)
        .filter(|word| {
            let mut current_index = 0;
            for letter in word.chars() {
                if let Some(index) = letters[current_index..].find(letter) {
                    current_index = index;
                } else {
                    return false;
                }
            }
            true
        })
        .map(|word| word.to_string())
        .collect()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_dictionary() {
        let path = "test.txt";
        let mut file = File::create(path).unwrap();
        file.write_all(b"one\ntwo\nthree\n").unwrap();
        assert_eq!(vec!["one", "two", "three"], load_dictionary(path));
        remove_file(path).unwrap();
    }

    #[test]
    fn test_find_swipe_words() {
        let dictionary: Vec<String> = vec![
            "answer".to_string(),
            "bee".to_string(),
            "cab".to_string(),
            "gaeing".to_string(),
            "gathering".to_string(),
            "gating".to_string(),
            "ghost".to_string(),
            "giant".to_string(),
            "gieing".to_string(),
            "gig".to_string(),
            "gigantic".to_string(),
            "giggle".to_string(),
            "going".to_string(),
            "goring".to_string(),
            "one".to_string(),
            "question".to_string(),
            "three".to_string(),
            "two".to_string(),
        ];
        assert_eq!(
            vec!["question"],
            find_swipe_words("qwertyuytresdftyuioknn", 5, &dictionary)
        );
        assert_eq!(
            vec!["gaeing", "gathering", "gating", "gieing", "going", "goring"],
            find_swipe_words("gijakjthoijerjidsdfnokg", 5, &dictionary)
        );
    }
}
