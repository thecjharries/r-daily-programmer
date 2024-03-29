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

fn sort_input<'a>(input: &Vec<&'a str>) -> Vec<&'a str> {
    let mut result = input.to_vec();
    result.sort();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_input() {
        let mut input = vec!["c", "b", "a"];
        let mut result = sort_input(&input);
        assert_eq!(input, vec!["c", "b", "a"]);
        assert_eq!(result, vec!["a", "b", "c"]);
        input = vec!["1", "11", "10", "5"];
        result = sort_input(&input);
        assert_eq!(input, vec!["1", "11", "10", "5"]);
        assert_eq!(result, vec!["1", "10", "11", "5"]);
    }
}
