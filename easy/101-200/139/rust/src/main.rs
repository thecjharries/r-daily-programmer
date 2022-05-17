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

use std::collections::BTreeMap;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn is_panagram(sentence: &str) -> bool {
    let mut letters: BTreeMap<char, u8> = BTreeMap::new();
    for c in sentence.to_lowercase().chars() {
        if c.is_alphabetic() {
            *letters.entry(c).or_insert(0) += 1;
        }
    }
    26 == letters.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(
            is_panagram("The quick brown fox jumps over the lazy dog."),
            true
        );
        assert_eq!(is_panagram("Pack my box with five dozen liquor jugs"), true);
        assert_eq!(
            is_panagram("Saxophones quickly blew over my jazzy hair"),
            false
        );
    }
}
