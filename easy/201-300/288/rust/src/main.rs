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

lazy_static! {
    static ref STOP_WORD_LIST: Vec<String> = vec![
        "I".to_string(),
        "a".to_string(),
        "about".to_string(),
        "an".to_string(),
        "and".to_string(),
        "are".to_string(),
        "as".to_string(),
        "at".to_string(),
        "be".to_string(),
        "by".to_string(),
        "com".to_string(),
        "for".to_string(),
        "from".to_string(),
        "how".to_string(),
        "in".to_string(),
        "is".to_string(),
        "it".to_string(),
        "of".to_string(),
        "on".to_string(),
        "or".to_string(),
        "that".to_string(),
        "the".to_string(),
        "this".to_string(),
        "to".to_string(),
        "was".to_string(),
        "what".to_string(),
        "when".to_string(),
        "where".to_string(),
        "who".to_string(),
        "will".to_string(),
        "with".to_string(),
        "the".to_string(),
    ];
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
