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

use itertools::Itertools;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn is_torn_number(input: u32) -> bool {
    if input > 999 && input < 10000 {
        let mut digits: Vec<u32> = Vec::new();
        let mut number = input;
        while 0 < number {
            digits.push(number % 10);
            number /= 10;
        }
        let unique_digits: Vec<u32> = digits.into_iter().unique().collect::<Vec<u32>>();
        if 4 == unique_digits.len() {
            let upper = unique_digits[0] * 10 + unique_digits[1];
            let lower = unique_digits[2] * 10 + unique_digits[3];
            return input == (upper + lower) * (upper + lower);
        }
    }
    false
}

fn find_torn_numbers() -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();
    for number in 1000..10000 {
        if is_torn_number(number) {
            result.push(number)
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_torn_number() {
        assert_eq!(false, is_torn_number(1111));
        assert_eq!(false, is_torn_number(111));
        assert_eq!(false, is_torn_number(11111));
        assert_eq!(false, is_torn_number(2025));
        assert_eq!(true, is_torn_number(3025));
        assert_eq!(true, is_torn_number(9801));
    }

    #[test]
    fn test_find_torn_numbers() {
        assert_eq!(vec![3025, 9801], find_torn_numbers());
    }
}
