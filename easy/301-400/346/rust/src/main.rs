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

use std::collections::HashMap;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn find_word_mapping(words: Vec<&str>) -> HashMap<char, usize> {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_word_mapping() {
        assert_eq!(
            HashMap::from_iter(vec![
                ('D', 7),
                ('E', 5),
                ('M', 1),
                ('N', 6),
                ('O', 0),
                ('R', 8),
                ('S', 9),
                ('Y', 2),
            ]),
            find_word_mapping(vec!["SEND", "MORE", "MONEY"])
        );
    }
}
