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

use num_to_words::integer_to_en_us;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn is_equation_palindrome(equation: &str) -> bool {
    let equation = equation.replace(" ", "");
    let equation_parts = equation.split('=').collect::<Vec<&str>>();
    if equation_parts.len() != 2 {
        return false;
    }
    let left_side = equation_parts[0];
    let right_side = equation_parts[1];
    let left_side_parts = left_side.split('+').collect::<Vec<&str>>();
    let right_side_parts = right_side.split('+').collect::<Vec<&str>>();
    let left_side_sum: u32 = left_side_parts
        .iter()
        .map(|part| part.parse::<u32>().unwrap())
        .sum();
    let right_side_sum: u32 = right_side_parts
        .iter()
        .map(|part| part.parse::<u32>().unwrap())
        .sum();
    if left_side_sum != right_side_sum {
        return false;
    }
    let mut left_side_chars: Vec<char> = Vec::new();
    for number in left_side_parts {
        for word in integer_to_en_us(number.parse::<i64>().unwrap())
            .unwrap()
            .split(' ')
        {
            left_side_chars.extend(word.chars());
        }
    }
    left_side_chars.sort();
    let mut right_side_chars: Vec<char> = Vec::new();
    for number in right_side_parts {
        for word in integer_to_en_us(number.parse::<i64>().unwrap())
            .unwrap()
            .split(' ')
        {
            right_side_chars.extend(word.chars());
        }
    }
    right_side_chars.sort();
    left_side_chars == right_side_chars
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(false, is_equation_palindrome("stub"));
        assert_eq!(true, is_equation_palindrome("1+12=11+2"));
        assert_eq!(true, is_equation_palindrome("1+22=2+21"));
        assert_eq!(false, is_equation_palindrome("1+22=2+22"));
    }
}
