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

fn main() {
    println!("rad");
}

fn find_possible_decodes(number: u64) -> Vec<String> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_possible_decodes() {
        assert_eq!(
            find_possible_decodes(123),
            vec!["abc".to_string(), "aw".to_string(), "lc".to_string()]
        );
        assert_eq!(
            find_possible_decodes(85121215),
            vec![
                "heababae".to_string(),
                "heababo".to_string(),
                "heabaue".to_string(),
                "heablae".to_string(),
                "heablo".to_string(),
                "heaubae".to_string(),
                "heaubo".to_string(),
                "heauue".to_string(),
                "helabae".to_string(),
                "helabo".to_string(),
                "helaue".to_string(),
                "hellae".to_string(),
                "hello".to_string()
            ]
        );
    }
}
