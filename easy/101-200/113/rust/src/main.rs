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
    static ref INT_PATTERN: Regex = Regex::new(r"^[+\-]?\d+$").unwrap();
    static ref FLOAT_PATTERN: Regex = Regex::new(r"^[+\-]?\d+\.\d+$").unwrap();
    static ref DATE_PATTERN: Regex = Regex::new(r"^\d{2}-\d{2}-\d{4S}$").unwrap();
}

enum InputKind {
    Int,
    Float,
    Date,
    Text,
}

fn main() {
    println!("rad");
}

fn determine_input_type(input: &str) -> InputKind {
    InputKind::Text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_input_type() {
        assert_eq!(determine_input_type("123"), InputKind::Int);
        assert_eq!(determine_input_type("+123"), InputKind::Int);
        assert_eq!(determine_input_type("-123"), InputKind::Int);
        assert_eq!(determine_input_type("123.456"), InputKind::Float);
        assert_eq!(determine_input_type("20-11-2012"), InputKind::Date);
        assert_eq!(determine_input_type("Hello, World!"), InputKind::Text);
        assert_eq!(determine_input_type("Hello 123"), InputKind::Text);
    }
}
