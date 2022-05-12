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

fn find_largest_divisible_by(n: u32, m: u32) -> Result<u128, String> {
    let start = u128::pow(10, n) - 1;
    if u128::from(m) > start {
        return Err(format!("m must be less than {}", start));
    }
    let end = u128::pow(10, n - 1);
    let mut current = start;
    while current >= end {
        if current % m as u128 == 0 {
            return Ok(current);
        }
        current -= 1;
    }
    Err(format!("no divisible by {} found", m))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_largest_divisible_by() {
        assert_eq!(find_largest_divisible_by(3, 2), Ok(998));
        assert!(find_largest_divisible_by(2, 101).is_err());
        assert_eq!(find_largest_divisible_by(7, 4241275), Ok(8482550));
    }
}
