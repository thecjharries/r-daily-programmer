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

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn censor_text(input: &str, words: Vec<String>) -> String {
    let mut result = input.to_string();
    for word in words {
        let replacement = format!(
            "{}{}",
            word.chars().nth(0).unwrap(),
            "*".repeat(word.len() - 1)
        );
        result = result.replace(&word, &replacement);
    }
    result
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_censor_text() {
        assert_eq!(
            "You j***face!",
            censor_text(
                "You jerkface!",
                vec!["jerk".to_string(), "test".to_string()]
            )
        );
        assert_eq!(
            "You j***!",
            censor_text("You jerk!", vec!["jerk".to_string(), "test".to_string()])
        );
    }
}
