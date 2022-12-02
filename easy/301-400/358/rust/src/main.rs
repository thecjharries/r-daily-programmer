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
use std::collections::HashMap;

lazy_static! {
    static ref SEVEN_DIGIT_TO_NUMBER: HashMap<String, char> = HashMap::from_iter([
        ("   \n  |\n  |".to_string(), '1'),
        (" _ \n _|\n|_ ".to_string(), '2'),
        (" _ \n _|\n _|".to_string(), '3'),
        ("   \n|_|\n  |".to_string(), '4'),
        (" _ \n|_ \n _|".to_string(), '5'),
        (" _ \n|_ \n|_|".to_string(), '6'),
        (" _ \n  |\n  |".to_string(), '7'),
        (" _ \n|_|\n|_|".to_string(), '8'),
        (" _ \n|_|\n _|".to_string(), '9'),
        (" _ \n| |\n|_|".to_string(), '0'),
    ]);
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn convert_seven_digit_to_number(seven_digit: &str) -> String {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_seven_digit_to_number() {
        assert_eq!("123456789".to_string(), convert_seven_digit_to_number("    _  _     _  _  _  _  _ \n  | _| _||_||_ |_   ||_||_|\n  ||_  _|  | _||_|  ||_| _|"));
        assert_eq!("433805825".to_string(), convert_seven_digit_to_number("    _  _  _  _  _  _  _  _ \n|_| _| _||_|| ||_ |_| _||_ \n  | _| _||_||_| _||_||_  _|"));
    }
}
