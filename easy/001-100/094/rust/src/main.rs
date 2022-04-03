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
    static ref ELEMENTS_FOR_EXERCISE: Vec<&'static str> =
        vec!["O", "P", "Y", "I", "Ra", "Pr", "Er", "Am"];
}

fn main() {
    println!("rad");
}

fn highlight(input: &str) -> Vec<String> {
    let mut results = Vec::new();
    for element in ELEMENTS_FOR_EXERCISE.iter() {
        let possible_index = input.to_lowercase().find(element.to_lowercase().as_str());
        match possible_index {
            Some(index) => {
                results.push(format!(
                    "{}[{}]{}",
                    &input[0..index],
                    element,
                    &input[index + element.len()..]
                ));
            }
            None => {}
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highlight() {
        assert_eq!(
            highlight("dailyprogrammer"),
            vec![
                "dailypr[O]grammer".to_string(),
                "daily[P]rogrammer".to_string(),
                "dail[Y]programmer".to_string(),
                "da[I]lyprogrammer".to_string(),
                "dailyprog[Ra]mmer".to_string(),
                "daily[Pr]ogrammer".to_string(),
                "dailyprogramm[Er]".to_string(),
                "dailyprogr[Am]mer".to_string()
            ]
        );
    }
}
