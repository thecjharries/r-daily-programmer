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

fn transpose_strings(input: Vec<String>) -> Vec<String> {
    let mut length = 0;
    for s in &input {
        if s.len() > length {
            length = s.len();
        }
    }
    let mut output = Vec::new();
    for _ in 0..length {
        output.push(String::new());
    }
    for string in &input {
        for index in 0..length {
            if index < string.len() {
                output[index].push(string.chars().nth(index).unwrap());
            } else {
                output[index].push(' ');
            }
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose_strings() {
        assert_eq!(
            transpose_strings(vec![String::from("abc"), String::from("def")]),
            vec![String::from("ad"), String::from("be"), String::from("cf")]
        );
    }
}
