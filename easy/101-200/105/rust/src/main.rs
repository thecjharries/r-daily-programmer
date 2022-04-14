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

fn unscramble_words(scrambled_words: Vec<&str>, word_list: Vec<String>) -> Vec<String> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        let words = BufReader::new(File::open("../enable1.txt").unwrap())
            .lines()
            .map(|l| l.expect("Could not read line"))
            .collect::<Vec<String>>();
        let scrambled_words = vec!["tac", "eeb", "ogd"];
        let expected: HashMap<String, Vec<string>> = HashMap::from_iter(vec![
            ("tac".to_string(), vec!["act", "cat"]),
            ("eeb".to_string(), vec!["bee"]),
            ("ogd".to_string(), vec!["dog", "god"]),
        ]);
        assert_eq!(unscramble_words(scrambled_words, words), expected);
    }
}
