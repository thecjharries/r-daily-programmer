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

use std::io::{Error, Write};

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn print_hello_world(output: &mut impl Write) -> Result<(), Error> {
    write!(output, "Hello, world!")?;
    Ok(())
}

fn find_numbers_divisible_by_3_and_5(max: u32) -> Vec<u32> {
    (1..max).filter(|n| n % 3 == 0 && n % 5 == 0).collect()
}

fn are_anagrams(first: &str, second: &str) -> bool {
    let mut first_chars = first.chars().collect::<Vec<char>>();
    let mut second_chars = second.chars().collect::<Vec<char>>();
    first_chars.sort();
    second_chars.sort();
    first_chars == second_chars
}

fn strip_character(input: &str, character: char) -> String {
    input.chars().filter(|c| *c != character).collect()
}

fn sum_vec(vec: &Vec<i32>) -> i32 {
    vec.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_hello_world() {
        let mut output: Vec<u8> = Vec::new();
        assert!(print_hello_world(&mut output).is_ok());
        assert_eq!(output, b"Hello, world!");
    }

    #[test]
    fn test_find_numbers_divisible_by_3_and_5() {
        assert_eq!(
            vec![15, 30, 45, 60, 75, 90],
            find_numbers_divisible_by_3_and_5(100)
        );
    }

    #[test]
    fn test_are_anagrams() {
        assert!(are_anagrams("dog", "god"));
        assert!(!are_anagrams("dog", "cab"));
    }

    #[test]
    fn test_strip_character() {
        assert_eq!("abcd", strip_character("abcde", 'e'));
    }

    #[test]
    fn test_sum_vec() {
        assert_eq!(15, sum_vec(&vec![1, 2, 3, 4, 5]));
    }
}
