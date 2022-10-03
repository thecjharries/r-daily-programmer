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

use std::error::Error;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn find_factorial(number: u128) -> Result<u128, Error> {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_factorial() {
        assert_eq!(5, find_factorial(120).unwrap());
        assert!(find_factorial(150).is_err());
        assert_eq!(10, find_factorial(3628800).unwrap());
        assert_eq!(12, find_factorial(479001600).unwrap());
        assert_eq!(3, find_factorial(6).unwrap());
        assert!(find_factorial(18).is_err());
        assert!(find_factorial(9).is_err());
    }
}
