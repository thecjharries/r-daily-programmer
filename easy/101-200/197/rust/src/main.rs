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

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref not_number_pattern: Regex = Regex::new(r"\D").unwrap();
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn is_valid_isbn(isbn: &str) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_isbn() {
        assert_eq!(false, is_valid_isbn("156881111"));
        assert_eq!(false, is_valid_isbn("15688111"));
        assert_eq!(false, is_valid_isbn("15688111111"));
        assert_eq!(false, is_valid_isbn("5885250576"));
        assert_eq!(true, is_valid_isbn("156881111X"));
        assert_eq!(true, is_valid_isbn("0-7475-3269-9"));
    }
}
