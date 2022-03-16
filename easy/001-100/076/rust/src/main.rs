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

fn titlecase(input: String, exceptions: Vec<String>) -> String {
    let mut first = true;
    input
        .to_lowercase()
        .split_whitespace()
        .map(|word| {
            if exceptions.contains(&word.to_string()) && !first {
                word.to_string()
            } else {
                first = false;
                word.chars().nth(0).unwrap().to_uppercase().to_string() + &word[1..]
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_titlecase() {
        assert_eq!(
            titlecase(
                "the quick brown fox jumps over the lazy dog".to_string(),
                vec!["jumps".to_string(), "the".to_string(), "over".to_string()]
            ),
            "The Quick Brown Fox jumps over the Lazy Dog".to_string()
        );
        assert_eq!(
            titlecase(
                "THE vitamins ARE IN my fresh CALIFORNIA raisins".to_string(),
                vec![
                    "are".to_string(),
                    "is".to_string(),
                    "in".to_string(),
                    "your".to_string(),
                    "my".to_string()
                ],
            ),
            "The Vitamins are in my Fresh California Raisins".to_string()
        )
    }
}
