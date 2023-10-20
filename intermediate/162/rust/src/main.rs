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

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn compress(input: &str) -> (Vec<String>, String) {
    todo!()
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
