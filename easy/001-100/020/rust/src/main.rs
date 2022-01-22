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

fn find_primes_below(max: u32) -> Vec<u32> {
    let mut primes: Vec<u32> = vec![2];
    for index in (3..max).step_by(2) {
        let mut is_prime: bool = true;
        for prime in &primes {
            if index % prime == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            primes.push(index);
        }
    }
    primes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_primes_below() {
        assert_eq!(find_primes_below(10), vec![2, 3, 5, 7]);
        assert_eq!(find_primes_below(20), vec![2, 3, 5, 7, 11, 13, 17, 19]);
    }
}
