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

use std::num::ParseIntError;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn parse_and_sum(first: char, second: char) -> Result<u32, ParseIntError> {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_sum() {
        assert_eq!(Ok(9), parse_and_sum('3', '6'));
        assert_eq!(Ok(13), parse_and_sum('4', '9'));
        assert_eq!(Ok(9), parse_and_sum('0', '9'));
        assert!(parse_and_sum('g', '6').is_err());
        assert!(parse_and_sum('7', 'h').is_err());
    }
}
