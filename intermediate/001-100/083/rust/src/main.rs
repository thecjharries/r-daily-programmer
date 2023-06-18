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

fn index(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    let contents = std::fs::read_to_string(filename).unwrap();
    for line in contents.lines() {
        let words = line.split_whitespace();
        for word in words {
            if !result.contains(&word.to_string()) {
                result.push(word.to_string());
            }
        }
    }
    result.sort();
    result.dedup();
    result
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index() {
        let filename = "./Makefile";
        let result = index(filename);
        assert!(0 < result.len());
    }
}
