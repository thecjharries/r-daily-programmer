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

fn convert_to_arabic(roman: &str) -> i64 {
    let mut result = 0;
    let mut roman = roman;
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
    while roman.len() > 0 {
        let mut found = false;
        for (roman_char, arabic) in roman_map.iter() {
            if roman.starts_with(roman_char) {
                result += *arabic;
                roman = &roman[roman_char.len()..];
                found = true;
                break;
            }
        }
        if !found {
            panic!("Invalid roman numeral: {}", roman);
        }
    }
    result
}

fn subtract_roman_numeral<'a>(first: &'a str, second: &'a str) -> String {
    let first_arabic = convert_to_arabic(first);
    let second_arabic = convert_to_arabic(second);
    let result_arabic = first_arabic - second_arabic;
    convert_to_roman(result_arabic)
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

    #[test]
    fn test_convert_to_arabic() {
        assert_eq!(convert_to_arabic("I"), 1);
        assert_eq!(convert_to_arabic("II"), 2);
        assert_eq!(convert_to_arabic("III"), 3);
        assert_eq!(convert_to_arabic("XIV"), 14);
    }

    #[test]
    fn test_subtract_roman_numeral() {
        assert_eq!(subtract_roman_numeral("X", "I"), "IX");
    }
}
