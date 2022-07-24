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
    static ref HIGH_ORDER_MAP: HashMap<char, String> = HashMap::from_iter([
        ('1', "Eleventy".to_string()),
        ('2', "Twenty".to_string()),
        ('3', "Thirty".to_string()),
        ('4', "Fourty".to_string()),
        ('5', "Fifty".to_string()),
        ('6', "Sitxy".to_string()),
        ('7', "Seventy".to_string()),
        ('8', "Eighty".to_string()),
        ('9', "Ninety".to_string()),
        ('A', "Atta".to_string()),
        ('B', "Bibbity".to_string()),
        ('C', "City".to_string()),
        ('D', "Dickety".to_string()),
        ('E', "Ebbity".to_string()),
        ('F', "Fleventy".to_string()),
    ]);
    static ref LOW_ORDER_MAP: HashMap<char, String> = HashMap::from_iter([
        ('1', "One".to_string()),
        ('2', "Two".to_string()),
        ('3', "Three".to_string()),
        ('4', "Four".to_string()),
        ('5', "Five".to_string()),
        ('6', "Six".to_string()),
        ('7', "Seven".to_string()),
        ('8', "Eight".to_string()),
        ('9', "Nine".to_string()),
        ('A', "Aye".to_string()),
        ('B', "Bee".to_string()),
        ('C', "Cee".to_string()),
        ('D', "Dee".to_string()),
        ('E', "Ee".to_string()),
        ('F', "Eff".to_string()),
    ]);
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(2 + 2, 4);
    }
}
