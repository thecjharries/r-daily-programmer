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

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
struct InvalidHandError;

#[cfg(not(tarpaulin_include))]
impl fmt::Display for InvalidHandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid scheme")
    }
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn convert_hand_to_base10(input: &str) -> Result<u32, InvalidHandError> {
    if 10 != input.len() {
        return Err(InvalidHandError);
    }
    let tens = input[0..5].to_string().trim_start_matches('0').to_string();
    let ones = input[5..10]
        .chars()
        .rev()
        .collect::<String>()
        .trim_start_matches('0')
        .to_string();
    if ones[..ones.len() - 1].contains("0") || tens[..tens.len() - 1].contains("0") {
        return Err(InvalidHandError);
    }
    let mut sum = ones.matches("1").count() as u32;
    if '1' == ones.chars().last().unwrap() {
        sum += 4;
    }
    sum += tens.matches("1").count() as u32 * 10;
    if '1' == tens.chars().last().unwrap() {
        sum += 40;
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_hand_to_base10() {
        assert_eq!(37, convert_hand_to_base10("0111011100").unwrap());
        assert_eq!(
            InvalidHandError,
            convert_hand_to_base10("1010010000").unwrap_err()
        );
        assert_eq!(73, convert_hand_to_base10("0011101110").unwrap());
        assert_eq!(55, convert_hand_to_base10("0000110000").unwrap());
        assert_eq!(
            InvalidHandError,
            convert_hand_to_base10("1111110001").unwrap_err()
        );
    }
}
