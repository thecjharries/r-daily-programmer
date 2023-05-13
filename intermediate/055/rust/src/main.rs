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

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ParseError {
    position: usize,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid number in position {}", self.position)
    }
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn parse_and_sum(first: char, second: char) -> Result<u32, ParseError> {
    let first_number = match first.to_digit(10) {
        Some(number) => number,
        None => return Err(ParseError { position: 1 }),
    };
    let second_number = match second.to_digit(10) {
        Some(number) => number,
        None => return Err(ParseError { position: 2 }),
    };
    Ok(first_number + second_number)
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_sum() {
        assert_eq!(Ok(9), parse_and_sum('3', '6'));
        assert_eq!(Ok(13), parse_and_sum('4', '9'));
        assert_eq!(Ok(9), parse_and_sum('0', '9'));
        assert!(parse_and_sum('g', '6').is_err());
        let error = parse_and_sum('7', 'h').unwrap_err();
        assert_eq!(2, error.position);
        assert_eq!("Invalid number in position 2", error.to_string());
        assert!(parse_and_sum('7', 'h').is_err());
    }
}
