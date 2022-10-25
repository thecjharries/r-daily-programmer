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
use std::collections::HashMap;

lazy_static! {
    static ref ENGLISH_TO_LEET: HashMap<char, String> = HashMap::from([
        ("A", "4".to_string()),
        ("B", "6".to_string()),
        ("E", "3".to_string()),
        ("I", "1".to_string()),
        ("L", "1".to_string()),
        ("M", "(V)".to_string()),
        ("N", "(\\)".to_string()),
        ("O", "0".to_string()),
        ("S", "5".to_string()),
        ("T", "7".to_string()),
        ("V", "\\/".to_string()),
        ("W", "`//".to_string()),
    ]);
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(2 + 2, 4);
    }
}
