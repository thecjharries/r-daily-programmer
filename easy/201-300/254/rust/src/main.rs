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
    static ref ATBASH_MAP: HashMap<char, char> = HashMap::from_iter(
        "abcdefghijklmnopqrstuvwxyz"
            .chars()
            .zip("zyxwvutsrqponmlkjihgfedcba".chars())
    );
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn atbash_encode(input: &str) -> String {
    input
        .to_lowercase()
        .chars()
        .map(|character| ATBASH_MAP.get(&character).unwrap_or(&character).to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atbash_encode() {
        assert_eq!("ullyzi".to_string(), atbash_encode("foobar"));
        assert_eq!("draziw".to_string(), atbash_encode("wizard"));
        assert_eq!(
            "/i/wzrobkiltiznnvi".to_string(),
            atbash_encode("/r/dailyprogrammer")
        );
        assert_eq!(
            "this is an example of the atbash cipher".to_string(),
            atbash_encode("gsrh rh zm vcznkov lu gsv zgyzhs xrksvi")
        );
    }
}
