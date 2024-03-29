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
    static ref NUMBERS_ONLY_PATTERN: Regex = Regex::new(r"^\d+$").unwrap();
}

fn main() {
    println!("rad");
}

fn is_only_numbers(input: &str) -> bool {
    NUMBERS_ONLY_PATTERN.is_match(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_only_numbers() {
        assert_eq!(is_only_numbers("123"), true);
        assert_eq!(is_only_numbers("123.45"), false);
        assert_eq!(is_only_numbers("abc"), false);
    }
}
