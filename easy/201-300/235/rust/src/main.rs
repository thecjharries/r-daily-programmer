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

fn get_distinct_prime_factors(number: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut current = number;
    if 0 == current % 2 {
        factors.push(2);
        while 0 == current % 2 {
            current /= 2;
        }
    }
    for i in (3..current).step_by(2) {
        if 0 == current % i {
            factors.push(i);
            while 0 == current % i {
                current /= i;
            }
        }
    }
    if 2 < current {
        factors.push(current);
    }
    factors
}

fn is_ruth_aaron_pair(a: u64, b: u64) -> bool {
    let a_factors = get_distinct_prime_factors(a);
    let b_factors = get_distinct_prime_factors(b);
    a_factors.sum::<u64>() == b_factors.sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_distinct_prime_factors() {
        assert_eq!(vec![23], get_distinct_prime_factors(23));
        assert_eq!(vec![2, 3], get_distinct_prime_factors(24));
        assert_eq!(vec![2, 3, 7, 17], get_distinct_prime_factors(714));
        assert_eq!(vec![5, 11, 13], get_distinct_prime_factors(715));
    }
}
