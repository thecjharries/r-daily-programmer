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
    static ref LETTERS: HashMap<char, String> = HashMap::from_iter(vec![('a', "4".to_string()),]);
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn translate(input: &str) -> String {
    input
        .chars()
        .map(|character| {
            LETTERS
                .get(&character.to_ascii_lowercase())
                .unwrap_or(&character.to_string())
                .to_string()
        })
        .collect()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!("r4d", translate("rad"));
        assert_eq!("R4D", translate("RAD"));
    }
}
