// Copyright 2023 CJ Harries
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

fn is_valid_luhn(number: &str) -> bool {
    let mut sum = 0;
    let mut is_odd = true;
    for character in number.chars().rev() {
        let digit = character.to_digit(10).unwrap();
        sum += if is_odd {
            digit
        } else {
            let doubled_digit = digit * 2;
            if doubled_digit > 9 {
                doubled_digit - 9
            } else {
                doubled_digit
            }
        };
        is_odd = !is_odd;
    }
    0 == sum % 10
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_luhn() {
        assert_eq!(true, is_valid_luhn("79927398713"));
        assert_eq!(true, is_valid_luhn("49927398716"));
        assert_eq!(false, is_valid_luhn("79927398714"));
    }
}
