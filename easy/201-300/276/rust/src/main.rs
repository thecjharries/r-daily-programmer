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

fn print_word_rectangle(word: &str, width: usize, height: usize) -> String {
    let end = word[1..word.len() - 1]
        .to_string()
        .chars()
        .rev()
        .collect::<String>();
    let search_space = word.to_string() + end.as_str();
    let mut output = String::new();
    for y in 0..(word.len() - 1) * height + 1 {
        for x in 0..(word.len() - 1) * width + 1 {
            if 0 == x % (word.len() - 1) || 0 == y % (word.len() - 1) {
                output.push(
                    search_space
                        .chars()
                        .nth((x + y) % search_space.len())
                        .unwrap(),
                );
            } else {
                output.push(' ');
            }
        }
        output.push('\n');
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_word_rectangle() {
        assert_eq!(
            "rekt\ne  k\nk  e\ntker\n",
            print_word_rectangle("rekt", 1, 1)
        );
        assert_eq!(
            "rektker\ne  k  e\nk  e  k\ntkerekt\nk  e  k\ne  k  e\nrektker\n",
            print_word_rectangle("rekt", 2, 2)
        );
    }
}
