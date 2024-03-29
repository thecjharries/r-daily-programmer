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

#[derive(Debug, PartialEq, Eq)]
enum InputType {
    Number,
    String,
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn determine_input_type(input: &str) -> InputType {
    if input.parse::<i32>().is_ok() {
        InputType::Number
    } else {
        if input.parse::<f32>().is_ok() {
            InputType::Number
        } else {
            InputType::String
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_input_type() {
        assert_eq!(InputType::Number, determine_input_type("123"));
        assert_eq!(InputType::Number, determine_input_type("44.234"));
        assert_eq!(InputType::String, determine_input_type("0x123N"));
    }
}
