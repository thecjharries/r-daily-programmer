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

fn generate_word_score(word: &str) -> u32 {
    let mut score = 0;
    for c in word.to_lowercase().chars() {
        score += c as u32 - 'a' as u32 + 1;
    }
    score
}

fn order_by_word_score(input: Vec<String>) -> Vec<String> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_word_score() {
        assert_eq!(generate_word_score("Hat"), 29);
        assert_eq!(generate_word_score("hat"), 29);
        assert_eq!(generate_word_score("Shoe"), 47);
    }

    #[test]
    fn test_order_by_word_score() {
        assert_eq!(
            order_by_word_score(vec!["Shoe", "hat"]),
            vec!["Hat", "Shoe"]
        );
    }
}
