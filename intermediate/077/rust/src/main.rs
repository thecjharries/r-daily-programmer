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

fn find_last_nonzero_digit_of_factorial(factorial: u128) -> u8 {
    let mut digit = 1;
    let mut number_of_fives = 0;
    for number in (2..=factorial).rev() {
        let mut current = number;
        while 0 == current % 5 {
            current /= 5;
            number_of_fives += 1;
        }
        while 0 < number_of_fives && 0 == current % 2 {
            current /= 2;
            number_of_fives -= 1;
        }
        digit = (digit * (current % 10)) % 10;
    }
    digit as u8
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_last_nonzero_digit_of_factorial() {
        assert_eq!(4, find_last_nonzero_digit_of_factorial(7));
        assert_eq!(8, find_last_nonzero_digit_of_factorial(10));
        assert_eq!(8, find_last_nonzero_digit_of_factorial(11));
        assert_eq!(6, find_last_nonzero_digit_of_factorial(12));
        assert_eq!(2, find_last_nonzero_digit_of_factorial(10_u128.pow(3)));
        assert_eq!(4, find_last_nonzero_digit_of_factorial(10_u128.pow(9)));
        // assert_eq!(6, find_last_nonzero_digit_of_factorial(10_u128.pow(100)));
    }
}
