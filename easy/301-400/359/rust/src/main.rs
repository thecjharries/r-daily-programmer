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
use memoize::memoize;
use std::collections::HashMap;

lazy_static! {
    static ref CONVERSION_MAP: HashMap<&'static str, &'static str> = HashMap::from_iter([
        ("1", "110"),
        ("0", "100"),
        ("11", "1101"),
        ("01", "1001"),
        ("10", "1100"),
        ("00", "1000"),
    ]);
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

#[memoize]
fn get_dragon_curve_term(term: usize) -> String {
    if 1 >= term {
        return "1".to_string();
    }
    let mut output = String::new();
    let previous = get_dragon_curve_term(term - 1);
    println!("{}", previous.len() / 2);
    for index in 0..previous.len() / 2 {
        output.push_str(
            CONVERSION_MAP
                .get(&previous[2 * index..2 * index + 2])
                .unwrap(),
        );
    }
    output.push_str(CONVERSION_MAP.get(&previous[previous.len() - 1..]).unwrap());
    output
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!("1".to_string(), get_dragon_curve_term(1));
        assert_eq!("110", get_dragon_curve_term(2));
        assert_eq!("1101100".to_string(), get_dragon_curve_term(3));
        assert_eq!("110110011100100".to_string(), get_dragon_curve_term(4));
        assert_eq!("1101100111001001110110001100100111011001110010001101100011001001110110011100100111011000110010001101100111001000110110001100100111011001110010011101100011001001110110011100100011011000110010001101100111001001110110001100100011011001110010001101100011001001110110011100100111011000110010011101100111001000110110001100100111011001110010011101100011001000110110011100100011011000110010001101100111001001110110001100100111011001110010001101100011001000110110011100100111011000110010001101100111001000110110001100100".to_string(), get_dragon_curve_term(9));
    }
}
