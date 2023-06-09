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

fn is_valid_roman(input: &str) -> bool {
    lazy_static! {
        static ref ROMAN_PATTERN: Regex = Regex::new(r"^(?P<thousands>M*)(?P<hundreds>C{0,3}|CD|DC{0,3}|CM)(?P<tens>X{0,3}|XL|LX{0,3}|XC)(?P<ones>I{0,3}|IV|VI{0,3}|IX)$").unwrap();
    }
    ROMAN_PATTERN.is_match(input)
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_roman() {
        assert!(is_valid_roman("MMXIX"));
        assert!(is_valid_roman("MCMXC"));
        assert!(is_valid_roman("MDCLXVI"));
        assert!(is_valid_roman("MMMDCCCLXXXVIII"));
        assert!(!is_valid_roman("IC"));
    }
}
