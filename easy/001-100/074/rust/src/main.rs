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

fn find_fibonacci_numbers_below(input: u64) -> Vec<u64> {
    let mut fibonacci_numbers = vec![1, 1];
    let mut current_fibonacci_number = 1;
    while current_fibonacci_number < input {
        let next_fibonacci_number = fibonacci_numbers[fibonacci_numbers.len() - 1]
            + fibonacci_numbers[fibonacci_numbers.len() - 2];
        fibonacci_numbers.push(next_fibonacci_number);
        current_fibonacci_number = next_fibonacci_number;
    }
    fibonacci_numbers.pop();
    fibonacci_numbers
}

fn find_zeckendorf_number(input: u64) -> Vec<u64> {
    let mut fibonacci_numbers = find_fibonacci_numbers_below(input);
    let mut zeckendorf_number = Vec::new();
    let mut current_number = input;
    while 0 < current_number {
        let current_fibonacci_number = fibonacci_numbers.pop().unwrap();
        if current_number >= current_fibonacci_number {
            zeckendorf_number.push(current_fibonacci_number);
            current_number -= current_fibonacci_number;
        }
    }
    zeckendorf_number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_fibonacci_numbers_below() {
        assert_eq!(
            find_fibonacci_numbers_below(100),
            vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89]
        );
    }

    #[test]
    fn test_find_zeckendorf_number() {
        assert_eq!(find_zeckendorf_number(100), vec![89, 8, 3]);
        assert_eq!(
            find_zeckendorf_number(14348907),
            vec![9227465, 3524578, 1346269, 196418, 46368, 6765, 987, 55, 2]
        );
    }
}
