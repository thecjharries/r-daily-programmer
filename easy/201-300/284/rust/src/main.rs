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

const DICTIONARY_PATH: &str = "/usr/share/dict/words";

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

fn find_swipe_words(letters: &str) -> Vec<String> {
    todo!()
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
}
