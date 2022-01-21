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
    static ref HEADER_PATTERN: Regex = Regex::new(r"(?ms).+?^\*\*\* START OF.+?$").unwrap();
    static ref FOOTER_PATTERN: Regex = Regex::new(r"(?ms)^\*\*\* END OF.+").unwrap();
    static ref ALLOWED_PATTERN: Regex = Regex::new(r"(?ims)[a-z0-9]").unwrap();
}

fn main() {
    println!("rad");
}

fn count_characters_in_sherlock(input: &str) -> usize {
    let mut cleaned: String = input.to_string();
    cleaned = HEADER_PATTERN.replace_all(&cleaned, "").to_string();
    cleaned = FOOTER_PATTERN.replace_all(&cleaned, "").to_string();
    ALLOWED_PATTERN.find_iter(&cleaned).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_characters_in_sherlock() {
        assert_eq!(count_characters_in_sherlock(""), 0);
        assert_eq!(count_characters_in_sherlock("a"), 1);
        assert_eq!(count_characters_in_sherlock("Project Gutenberg's The Adventures of Sherlock Holmes, by Arthur Conan Doyle

This eBook is for the use of anyone anywhere at no cost and with
almost no restrictions whatsoever.  You may copy it, give it away or
re-use it under the terms of the Project Gutenberg License included
with this eBook or online at www.gutenberg.org


Title: The Adventures of Sherlock Holmes

Author: Arthur Conan Doyle

Posting Date: April 18, 2011 [EBook #1661]
First Posted: November 29, 2002

Language: English

Character set encoding: ASCII

*** START OF THIS PROJECT GUTENBERG EBOOK THE ADVENTURES OF SHERLOCK HOLMES ***"), 0);
        assert_eq!(count_characters_in_sherlock("Arthur Conan Doyle

*** END OF THIS PROJECT GUTENBERG EBOOK THE ADVENTURES OF SHERLOCK HOLMES ***

***** This file should be named 1661.txt or 1661.zip *****"), 16);
    }
}
