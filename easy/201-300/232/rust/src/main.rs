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
use regex::Regex;

lazy_static! {
    static ref NOT_LETTERS_PATTERN: Regex = Regex::new(r"[^a-z]").unwrap();
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn is_palindrome(input: &str) -> bool {
    let chars = NOT_LETTERS_PATTERN
        .replace_all(&input.to_lowercase(), "")
        .chars()
        .collect::<Vec<char>>();
    for index in 0..chars.len() / 2 {
        println!("{} {}", chars[index], chars[chars.len() - index - 1]);
        if chars[index] != chars[chars.len() - index - 1] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(true, is_palindrome("Was it a car\nor a cat\nI saw?"));
        assert_eq!(false, is_palindrome("rad"));
        assert_eq!(
            false,
            is_palindrome("A man, a plan, \na canal, a hedgehog, \na podiatrist, \nPanama!")
        );
    }
}
