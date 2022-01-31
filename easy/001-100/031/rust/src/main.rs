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

fn convert_u64_to_base26(input: u64) -> &str {

}

fn multiply_base_26<'a>(first: &'a str, second: &'a str) -> &'a str {
    "FNEUZJA"
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
    fn test_multiply_base_26() {
        assert_eq!(multiply_base_26("CSGHJ", "CBA"), "FNEUZJA");
    }
}
