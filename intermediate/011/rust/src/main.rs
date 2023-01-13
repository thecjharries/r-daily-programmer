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

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref NUMBER_MAP: HashMap<char, char> = {
        let mut map = HashMap::new();
        map.insert('0', '0');
        map.insert('1', '1');
        map.insert('2', '2');
        map.insert('5', '5');
        map.insert('6', '9');
        map.insert('8', '8');
        map.insert('9', '6');
        map
    };
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn is_upside_down_number(number: u32) -> bool {
    let number_chars = number.to_string().chars().collect::<Vec<char>>();
    let mut flipped: Vec<char> = Vec::new();
    for number_char in number_chars {
        if !NUMBER_MAP.contains_key(&number_char) {
            return false;
        }
        flipped.push(NUMBER_MAP[&number_char]);
    }
    flipped.reverse();
    let flipped_string = flipped.iter().collect::<String>();
    flipped_string == number.to_string()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_upside_down_number() {
        assert_eq!(true, is_upside_down_number(69));
        assert_eq!(true, is_upside_down_number(96));
        assert_eq!(true, is_upside_down_number(101));
        assert_eq!(true, is_upside_down_number(111));
        assert_eq!(true, is_upside_down_number(181));
        assert_eq!(true, is_upside_down_number(609));
        assert_eq!(true, is_upside_down_number(1961));
        assert_eq!(false, is_upside_down_number(12));
    }
}
