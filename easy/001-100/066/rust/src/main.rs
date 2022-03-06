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

fn convert_to_roman(input: i64) -> String {
    let mut result = String::new();
    let mut input = input;
    let mut roman_map = [
        ("M", 1000),
        ("CM", 900),
        ("D", 500),
        ("CD", 400),
        ("C", 100),
        ("XC", 90),
        ("L", 50),
        ("XL", 40),
        ("X", 10),
        ("IX", 9),
        ("V", 5),
        ("IV", 4),
        ("I", 1),
    ];
    for (roman, arabic) in roman_map.iter() {
        while input >= *arabic {
            result.push_str(roman);
            input -= *arabic;
        }
    }
    result
}

fn subtract_roman_numeral<'a>(first: &'a str, second: &'a str) -> &'a str {
    &""
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_roman() {
        assert_eq!(convert_to_roman(1), "I");
        assert_eq!(convert_to_roman(2), "II");
        assert_eq!(convert_to_roman(3), "III");
        assert_eq!(convert_to_roman(14), "XIV");
    }
}
