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
    static ref LINE_PATTERN: Regex = Regex::new(r"^\s*(.*?)\s*$").unwrap();
}

fn main() {
    println!("rad");
}

fn find_length_of_longest_line(lines: &Vec<&str>) -> usize {
    lines
        .iter()
        .map(|line| line.len())
        .max()
        .unwrap()
}

fn left_justify(lines: &Vec<&str>) -> Vec<String> {
    let mut result = Vec::new();
    for line in lines {
        let mut justified_line = String::new();
        match LINE_PATTERN.captures(line) {
            Some(capture) => {
                justified_line.push_str(&capture[1]);
            },
            None => unreachable!(),
        }
        result.push(justified_line);
    }
    result
}

fn right_justify(lines: &Vec<&str>) -> Vec<String> {
    let max_length = find_length_of_longest_line(lines);
    let mut result: Vec<String> = Vec::new();
    for line in lines {
        let mut justified_line = String::new();
        match LINE_PATTERN.captures(line) {
            Some(capture) => {
                println!("'{}'", &capture[1]);
                let line_length = capture[1].len();
                justified_line.push_str(&" ".repeat(max_length - line_length));
                justified_line.push_str(&capture[1]);
            },
            None => unreachable!(),
        }
        result.push(justified_line);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_length_of_longest_line() {
        assert_eq!(find_length_of_longest_line(&vec!["a", "bb", "ccc"]), 3);
        assert_eq!(find_length_of_longest_line(&vec!["a", "bb", "ccc", "dddd"]), 4);
        assert_eq!(find_length_of_longest_line(&vec!["a", "bb", "ccc", "dddd", "eeeee"]), 5);
    }

    #[test]
    fn test_left_justify() {
        assert_eq!(left_justify(&vec!["  a", " bb", "ccc"]), vec!["a", "bb", "ccc"]);
        assert_eq!(left_justify(&vec!["a", " bb", "ccc", "dddd"]), vec!["a", "bb", "ccc", "dddd"]);
        assert_eq!(left_justify(&vec!["a", "bb", "  ccc", "dddd", "eeeee"]), vec!["a", "bb", "ccc", "dddd", "eeeee"]);
    }

    #[test]
    fn test_right_justify() {
        assert_eq!(right_justify(&vec![" a ", " bb", "ccc"]), vec!["  a", " bb", "ccc"]);
        assert_eq!(right_justify(&vec!["a", " bb", "ccc", "dddd"]), vec!["   a", "  bb", " ccc", "dddd"]);
        assert_eq!(right_justify(&vec!["a  ", " bb ", "  ccc", "dddd ", "eeeee"]), vec!["    a", "   bb", "  ccc", " dddd", "eeeee"]);
    }
}
