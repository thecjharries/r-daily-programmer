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

use regex::Regex;

fn main() {
    println!("rad");
}

fn is_valid_phone_number(input: &str) -> bool {
    let phone_number_pattern : Regex = Regex::new(r"^(?:\d{10}|(?:\d{3}[.-])?\d{3}[.-]\d{4}|\(\d{3}\) ?\d{3}-\d{4})$").unwrap();
    phone_number_pattern.is_match(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_phone_number() {
        assert_eq!(is_valid_phone_number("1234567890"), true);
        assert_eq!(is_valid_phone_number("123-456-7890"), true);
        assert_eq!(is_valid_phone_number("123.456.7890"), true);
        assert_eq!(is_valid_phone_number("(123)456-7890"), true);
        assert_eq!(is_valid_phone_number("(123) 456-7890"), true);
        assert_eq!(is_valid_phone_number("456-7890"), true);
        assert_eq!(is_valid_phone_number("123-45-6789"), false);
        assert_eq!(is_valid_phone_number("123:4567890"), false);
        assert_eq!(is_valid_phone_number("123/456-7890"), false);
    }
}
