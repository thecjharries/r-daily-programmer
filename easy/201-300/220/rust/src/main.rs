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

fn organize_sentence(input: &str) -> String {
    let exploded = input.split_whitespace().collect::<Vec<_>>();
    let mut output: Vec<String> = Vec::new();
    for word in exploded {
        let chars = word.chars();
        let mut punctuation: HashMap<usize, char> = HashMap::new();
        let mut unsorted = Vec::new();
        for (index, char) in chars.enumerate() {
            if char.is_alphabetic() {
                unsorted.push(char.to_ascii_lowercase());
            } else {
                punctuation.insert(index, char);
            }
        }
        unsorted.sort_by(|a, b| b.cmp(a));
        println!("{:?}", unsorted);
        let mut sorted = String::new();
        let mut index = 0;
        while index < word.len() {
            if let Some(punctuation_char) = punctuation.get(&index) {
                sorted.push(*punctuation_char);
            } else {
                let char = unsorted.pop().unwrap();
                println!("{:?}", char);
                if word.chars().nth(index).unwrap().is_uppercase() {
                    sorted.push(char.to_uppercase().next().unwrap());
                } else {
                    sorted.push(char);
                }
            }
            index += 1;
        }
        output.push(sorted);
    }
    output.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_organize_sentence() {
        assert_eq!(
            "Hist aceeghlln denos't eems os adhr.",
            organize_sentence("This challenge doesn't seem so hard.")
        );
        assert_eq!("Eehrt aer emor ghinst beeentw aeehnv adn aehrt, Ahioort, ahnt aer ademrt fo in oruy hhilooppsy.", organize_sentence("There are more things between heaven and earth, Horatio, than are dreamt of in your philosophy."));
    }
}
