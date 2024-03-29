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

fn is_kaprekar_number(input: u32) -> bool {
    if 9 > input {
        return false;
    }
    let square = input * input;
    let square_string = square.to_string();
    let square_string_length = square_string.len();
    let first_half = square_string[..square_string_length / 2]
        .parse::<u32>()
        .unwrap();
    let second_half = square_string[square_string_length / 2..]
        .parse::<u32>()
        .unwrap();
    input == first_half + second_half
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_kaprekar_number() {
        assert!(is_kaprekar_number(9));
        assert!(is_kaprekar_number(45));
        assert!(is_kaprekar_number(55));
        assert!(is_kaprekar_number(99));
        assert!(is_kaprekar_number(297));
        assert!(!is_kaprekar_number(1));
        assert!(!is_kaprekar_number(2));
        assert!(!is_kaprekar_number(3));
        assert!(!is_kaprekar_number(296));
    }
}
