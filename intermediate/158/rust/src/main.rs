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

fn generate_construction(input: &str) -> String {
    let mut results = Vec::new();
    for _ in 0..10 {
        results.push(String::new());
    }
    let mut height = 0;
    for character in input.chars() {
        if character.is_alphabetic() {
            results[height].push(character);
            for index in height + 1..10 {
                results[index].push(' ');
            }
            height = 0;
        } else if character.is_numeric() {
            height = character.to_digit(10).unwrap() as usize;
            for index in 0..height {
                results[index].push(' ');
            }
        }
    }
    results.reverse();
    results.join("\n")
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(" \n \n \n \n \n \n \n \n \na", generate_construction("a"));
        assert_eq!("           \n           \n           \n           \n  a        \n          h\n g         \n           \n           \nj  jbcdefg ", generate_construction("j3g5ajbcdefg4h"))
    }
}
