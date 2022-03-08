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

fn is_prime(number: i64, primes: &Vec<i64>) -> bool {
    for prime in primes {
        if number == *prime {
            return true;
        }
        if number % prime == 0 {
            return false;
        }
    }
    for index in (primes[primes.len() - 1]..((number as f64).sqrt().ceil() as i64)).step_by(2) {
        if number % index == 0 {
            return false;
        }
    }
    true
}

fn is_palindrome(number: i64) -> bool {
    let string_representation = format!("{}", number);
    string_representation == string_representation.chars().rev().collect::<String>()
}

fn find_emirps_below(number: i64) -> Vec<i64> {
    let mut primes = vec![2, 3, 5, 7, 11];
    let mut emirps = Vec::new();
    for index in (13..number).step_by(2) {
        if is_prime(index, &primes) {
            primes.push(index);
        }
    }
    for prime in &primes {
        if !is_palindrome(*prime) {
            let reversed_string = format!("{}", prime).chars().rev().collect::<String>();
            if is_prime(reversed_string.parse::<i64>().unwrap(), &primes) {
                emirps.push(*prime);
            }
        }
    }
    emirps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_emirps_below() {
        assert_eq!(find_emirps_below(100), vec![13, 17, 31, 37, 71, 73, 79, 97]);
    }

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(2, &vec![2]), true);
        assert_eq!(is_prime(3, &vec![2]), true);
        assert_eq!(is_prime(4, &vec![2]), false);
        assert_eq!(is_prime(5, &vec![2, 3]), true);
    }

    #[test]
    fn test_is_palindrome() {
        assert_eq!(is_palindrome(1), true);
        assert_eq!(is_palindrome(11), true);
        assert_eq!(is_palindrome(12), false);
        assert_eq!(is_palindrome(12321), true);
        assert_eq!(is_palindrome(12322), false);
    }
}
