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
    static ref NEXT_ALLOWED_CUT: HashMap<String, Vec<String>> = HashMap::from_iter([
        (
            "white".to_string(),
            vec![
                "red".to_string(),
                "orange".to_string(),
                "green".to_string(),
                "purple".to_string()
            ]
        ),
        ("red".to_string(), vec!["green".to_string()]),
        (
            "black".to_string(),
            vec!["red".to_string(), "purple".to_string(), "black".to_string()]
        ),
        (
            "orange".to_string(),
            vec!["red".to_string(), "black".to_string()]
        ),
        (
            "green".to_string(),
            vec!["orange".to_string(), "white".to_string()]
        ),
        (
            "purple".to_string(),
            vec!["red".to_string(), "black".to_string()]
        ),
    ]);
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn did_bomb_explode(cuts: Vec<String>) -> bool {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_did_bomb_explode() {
        assert_eq!(
            false,
            did_bomb_explode(vec![
                "white".to_string(),
                "red".to_string(),
                "green".to_string(),
                "white".to_string()
            ])
        );
        assert_eq!(
            true,
            did_bomb_explode(vec![
                "white".to_string(),
                "orange".to_string(),
                "green".to_string(),
                "white".to_string()
            ])
        );
        assert_eq!(
            true,
            did_bomb_explode(vec![
                "qqq".to_string(),
                "white".to_string(),
                "orange".to_string(),
                "green".to_string(),
                "white".to_string()
            ])
        );
        assert_eq!(
            true,
            did_bomb_explode(vec![
                "white".to_string(),
                "qqq".to_string(),
                "orange".to_string(),
                "green".to_string(),
                "white".to_string()
            ])
        );
    }
}
