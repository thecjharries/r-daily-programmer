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

fn build_word_snake(words: Vec<String>) -> String {
    let mut output = Vec::new();
    let mut current_spacing = 0;
    let mut index = 0;
    let last_word = words[words.len() - 1].as_str().clone();
    for word in words.iter() {
        if 0 == index % 2 {
            output.push(" ".repeat(current_spacing) + &word);
            current_spacing += word.len() - 1;
        } else {
            let chars = &word.chars().collect::<Vec<_>>()[1..word.len() - 1];
            for char in chars.iter() {
                let mut new_word = String::new();
                new_word.push_str(&" ".repeat(current_spacing));
                new_word.push(*char);
                output.push(new_word);
            }
        }
        index += 1;
    }
    if 0 == index % 2 {
        output.push(" ".repeat(current_spacing) + &last_word[last_word.len() - 1..].to_string());
    }
    output.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(
            "SHENANIGANS
          A
          L
          T
          YOUNGSTER",
            build_word_snake(vec![
                "SHENANIGANS".to_string(),
                "SALTY".to_string(),
                "YOUNGSTER".to_string()
            ])
        );
        assert_eq!(
            "SHENANIGANS
          A
          L
          T
          Y",
            build_word_snake(vec!["SHENANIGANS".to_string(), "SALTY".to_string()])
        );
    }
}
