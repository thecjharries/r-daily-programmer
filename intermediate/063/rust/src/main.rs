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
fn prng(n: u64) -> u64 {
    if 0 == n {
        123456789
    } else {
        (prng(n - 1) * 22695477 + 12345) % 1073741824
    }
}

fn reverse(count: usize, input: Vec<i64>) -> Vec<i64> {
    let mut output = input.clone();
    for i in 0..count {
        output[i] = input[count - i - 1];
    }
    output
}

fn reverse_sort(input: Vec<u64>) -> Vec<u64> {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prng() {
        assert_eq!(123456789, prng(0));
        assert_eq!(752880530, prng(1));
        assert_eq!(826085747, prng(2));
        assert_eq!(576968456, prng(3));
        assert_eq!(721429729, prng(4));
        assert_eq!(151520653, prng(1000));
        assert_eq!(65237510, prng(9997));
        assert_eq!(921739127, prng(9998));
        assert_eq!(926774748, prng(9999));
    }

    #[test]
    fn test_reverse() {
        assert_eq!(reverse(1, vec![1, 2, 3, 4, 5]), vec![1, 2, 3, 4, 5]);
        assert_eq!(reverse(2, vec![1, 2, 3, 4, 5]), vec![2, 1, 3, 4, 5]);
        assert_eq!(reverse(3, vec![1, 2, 3, 4, 5]), vec![3, 2, 1, 4, 5]);
        assert_eq!(reverse(4, vec![1, 2, 3, 4, 5]), vec![4, 3, 2, 1, 5]);
        assert_eq!(reverse(5, vec![1, 2, 3, 4, 5]), vec![5, 4, 3, 2, 1]);
    }
}
