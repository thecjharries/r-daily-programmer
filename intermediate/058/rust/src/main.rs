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

use num_bigint::BigUint;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn find_next_palindrome(input: BigUint) -> BigUint {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_find_next_palindrome() {
        assert_eq!(
            BigUint::from(101u32),
            find_next_palindrome(BigUint::from(99u32))
        );
        assert_eq!(
            BigUint::from(818u32),
            find_next_palindrome(BigUint::from(808u32))
        );
        assert_eq!(
            BigUint::from(1001u32),
            find_next_palindrome(BigUint::from(999u32))
        );
        assert_eq!(
            BigUint::from(2222u32),
            find_next_palindrome(BigUint::from(2133u32))
        );
        assert_eq!(
            BigUint::from(8998u32),
            find_next_palindrome(BigUint::from(8888u32))
        );
        assert_eq!(
            BigUint::from_str("4052555153515552504").unwrap(),
            find_next_palindrome(BigUint::from(3u8).pow(39))
        );
        assert_eq!(
            BigUint::from_str("3234476509624757991344647769100216810857204027580186120019677464431997574269056744323").unwrap(),
            find_next_palindrome(BigUint::from(7u8).pow(100))
        );
    }
}
