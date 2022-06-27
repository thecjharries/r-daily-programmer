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

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn find_at_handles(words: Vec<String>) -> Vec<String> {
    words
        .iter()
        .filter(|w| w.starts_with("at") && 2 < w.len())
        .map(|w| w.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_at_handles() {
        let words = vec![
            "asynchrony".to_string(),
            "asyndeta".to_string(),
            "asyndetic".to_string(),
            "asyndetically".to_string(),
            "asyndeton".to_string(),
            "asyndetons".to_string(),
            "at".to_string(),
            "atabal".to_string(),
            "atabals".to_string(),
            "atactic".to_string(),
            "ataghan".to_string(),
            "ataghans".to_string(),
        ];
        let expected = vec![
            "atabal".to_string(),
            "atabals".to_string(),
            "atactic".to_string(),
            "ataghan".to_string(),
            "ataghans".to_string(),
        ];
        assert_eq!(expected, find_at_handles(words));
    }
}
