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
    static ref BRAILLE_TO_ROMAN: HashMap<&'static str, &'static str> = HashMap::from_iter[
        ("O.....", "a"),
        ("O.O...", "b"),
        ("OO....", "c"),
        ("OO.O..", "d"),
        ("O..O..", "e"),
        ("OOO...", "f"),
        ("OOOO..", "g"),
        ("O.OO..", "h"),
        (".OO...", "i"),
        (".OOO..", "j"),
        ("O...O.", "k"),
        ("O.O.O.", "l"),
        ("OO..O.", "m"),
        ("OO.OO.", "n"),
        ("O..OO.", "o"),
        ("OOO.O.", "p"),
        ("OOOOO.", "q"),
        ("O.OOO.", "r"),
        (".OO.O.", "s"),
        (".OOOO.", "t"),
        ("O...OO", "u"),
        ("O.O.OO", "v"),
        (".OOO.O", "w"),
        ("OO..OO", "x"),
        ("OO.OOO", "y"),
        ("O..OOO", "z"),
    ];
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn convert_braille_to_roman(input: &str) -> String {
    // String::from_iter(input.chars().filter_map(|c| BRAILLE_TO_ROMAN.get(&c.to_string())))
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_braille_to_roman() {
        assert_eq(
            convert_braille_to_roman(
                "O. O. O. O. O. .O O. O. O. OO
OO .O O. O. .O OO .O OO O. .O
.. .. O. O. O. .O O. O. O. ..",
            ),
            "helloworld",
        )
    }
}
