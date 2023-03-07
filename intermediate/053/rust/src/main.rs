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

use memoize::memoize;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

#[memoize]
fn prng(input: u64) -> u64 {
    if 0 == input {
        return 123456789;
    }
    (22695477 * prng(input - 1) + 12345) % 1073741824
}

fn sum_largest(iterations: u64, largest_count: usize) -> u64 {
    let mut results = Vec::new();
    for index in 0..iterations {
        results.push(prng(index));
    }
    results.sort();
    results[iterations as usize - largest_count..].iter().sum()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prng() {
        assert_eq!(123456789, prng(0));
        assert_eq!(752880530, prng(1));
        assert_eq!(1048156299, prng(10));
    }

    #[test]
    fn test_sum_largest() {
        assert_eq!(1073683936567, sum_largest(10000000, 1000));
    }
}
