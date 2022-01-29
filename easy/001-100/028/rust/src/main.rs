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

fn find_duplicates(input: Vec<i32>) -> Vec<i32> {
    let mut numbers = input.clone();
    numbers.sort();
    let mut result = Vec::new();
    let mut current = numbers[0];
    for i in 1..numbers.len() {
        if numbers[i] == current {
            result.push(numbers[i]);
        } else {
            current = numbers[i];
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        let mut input = (0..100000).collect::<Vec<i32>>();
        input.push(10);
        input.push(100);
        input.push(1000);
        input.push(10000);
        input.push(100000);
        assert_eq!(find_duplicates(input), vec![10, 100, 1000, 10000, 100000]);
    }
}
