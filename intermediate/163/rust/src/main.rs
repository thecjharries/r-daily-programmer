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

fn compare_words(first: &str, second: &str) -> usize {
    let mut count = 0;
    for (first_character, second_character) in first.chars().zip(second.chars()) {
        if first_character == second_character {
            count += 1;
        }
    }
    count
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_words_generates_similarity_score() {
        assert_eq!(0, compare_words("a", "b"));
        assert_eq!(1, compare_words("a", "a"));
        assert_eq!(2, compare_words("rad", "bad"));
        assert_eq!(0, compare_words("abc", "cab"));
    }
}
