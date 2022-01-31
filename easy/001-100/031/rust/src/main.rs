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

fn main() {
    println!("rad");
}

fn convert_char_to_u8(input: char) -> u8 {
    input as u8 - 'a' as u8
}

fn convert_base26_to_u64(input: &str) -> u64 {
    let mut result: u64 = 0;
    let mut multiplier: u64 = 1;
    for c in input.to_lowercase().chars().rev() {
        result += multiplier * (c as u64 - 'a' as u64);
        multiplier *= 26;
    }
    result
}

fn convert_u64_to_base26(input: u64) -> String {
    let mut result: String = String::new();
    let mut remainder: u64 = input;
    if remainder == 0 {
        result.push('a');
    }
    while remainder > 0 {
        let digit = remainder % 26;
        result.push((digit + 'a' as u64) as u8 as char);
        remainder /= 26;
    }
    result.chars().rev().collect::<String>().to_uppercase()
}

fn multiply_base_26(first: &str, second: &str) -> String {
    let first_u64 = convert_base26_to_u64(first);
    let second_u64 = convert_base26_to_u64(second);
    convert_u64_to_base26(first_u64 * second_u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_char_to_u8() {
        assert_eq!(convert_char_to_u8('a'), 0);
        assert_eq!(convert_char_to_u8('z'), 25);
    }

    #[test]
    fn test_convert_base26_to_u64() {
        assert_eq!(convert_base26_to_u64("a"), 0);
        assert_eq!(convert_base26_to_u64("CSGHJ"), 1234567);
        assert_eq!(convert_base26_to_u64("CBA"), 1378);
        assert_eq!(convert_base26_to_u64("FNEUZJA"), 1701233326);
    }

    #[test]
    fn test_convert_u64_to_base26() {
        assert_eq!(convert_u64_to_base26(0), "A");
        assert_eq!(convert_u64_to_base26(1234567), "CSGHJ");
        assert_eq!(convert_u64_to_base26(1378), "CBA");
        assert_eq!(convert_u64_to_base26(1701233326), "FNEUZJA");
    }

    #[test]
    fn test_multiply_base_26() {
        assert_eq!(multiply_base_26("CSGHJ", "CBA"), "FNEUZJA".to_string());
    }
}
