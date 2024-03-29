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
    static ref PROMPT_PATTERN: Regex = Regex::new(r".?\*+.?").unwrap();
}

fn main() {
    println!("rad");
}

fn remove_per_prompt(input: &str) -> String {
    PROMPT_PATTERN.replace_all(input, "").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_per_prompt() {
        assert_eq!(remove_per_prompt("adf*lp"), "adp".to_string());
        assert_eq!(remove_per_prompt("a*o"), "".to_string());
        assert_eq!(remove_per_prompt("*dech*"), "ec".to_string());
        assert_eq!(remove_per_prompt("de**po"), "do".to_string());
        assert_eq!(remove_per_prompt("sa*n*ti"), "si".to_string());
        assert_eq!(remove_per_prompt("abc"), "abc".to_string());
    }
}
