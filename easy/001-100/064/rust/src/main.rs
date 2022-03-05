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

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn find_divisors(number: u64) -> Vec<u64> {
    let mut divisors = Vec::new();
    for index in 1..((number as f64).sqrt() as u64 + 1) {
        if number % index == 0 {
            divisors.push(index);
            if index != number / index {
                divisors.push(number / index);
            }
        }
    }
    divisors.sort();
    divisors
}

fn find_divisor_sum(number: u64) -> u64 {
    find_divisors(number).iter().sum()
}

fn find_divisor_count(number: u64) -> u64 {
    find_divisors(number).len() as u64
}

fn find_totatives(number: u64) -> Vec<u64> {
    let mut totatives = Vec::new();
    for index in 1..number {
        if gcd(number, index) == 1 {
            totatives.push(index);
        }
    }
    totatives
}

fn find_totient(number: u64) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(0, 0), 0);
        assert_eq!(gcd(0, 1), 1);
        assert_eq!(gcd(1, 0), 1);
        assert_eq!(gcd(1, 1), 1);
        assert_eq!(gcd(2, 2), 2);
        assert_eq!(gcd(2, 4), 2);
        assert_eq!(gcd(4, 2), 2);
    }

    #[test]
    fn test_find_divisors() {
        assert_eq!(
            find_divisors(60),
            vec![1, 2, 3, 4, 5, 6, 10, 12, 15, 20, 30, 60]
        );
    }

    #[test]
    fn test_find_divisor_sum() {
        assert_eq!(
            find_divisor_sum(60),
            1 + 2 + 3 + 4 + 5 + 6 + 10 + 12 + 15 + 20 + 30 + 60
        );
    }

    #[test]
    fn test_find_divisor_count() {
        assert_eq!(find_divisor_count(60), 12);
    }

    #[test]
    fn test_find_totatives() {
        assert_eq!(find_totatives(30), vec![1, 7, 11, 13, 17, 19, 23, 29]);
    }

    #[test]
    fn test_find_totient() {
        assert_eq!(find_totient(30), 8);
    }
}
