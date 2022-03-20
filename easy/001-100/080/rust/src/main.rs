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

use std::fs;

fn main() {
    println!("rad");
}

fn load_dictionary(file_path: String) -> Vec<String> {
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    contents.lines().map(|x| x.to_string()).collect()
}

fn find_anagrams(word: String, dictionary: Vec<String>) -> Vec<String> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_dictionary() {
        assert_eq!(load_dictionary("../enable1.txt".to_string()).len(), 172820);
    }
}
