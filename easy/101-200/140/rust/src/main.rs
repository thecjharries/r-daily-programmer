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

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn to_camel_case(input: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;
    for c in input.chars() {
        if '_' == c || ' ' == c {
            capitalize_next = true;
        } else {
            if capitalize_next {
                result.push(c.to_ascii_uppercase());
            } else {
                result.push(c.to_ascii_lowercase());
            }
            capitalize_next = false;
        }
    }
    result
}

fn to_snake_case(input: &str, upper: bool) -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_camel_case() {
        assert_eq!(to_camel_case("user id"), "userId");
    }

    #[test]
    fn test_to_snake_case() {
        assert_eq!(to_snake_case("user id", true), "USER_ID");
        assert_eq!(to_snake_case("user id", false), "user_id");
    }
}
