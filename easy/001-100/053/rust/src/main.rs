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

fn combine_two_sorted_vectors(first: Vec<i32>, second: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut first_index = 0;
    let mut second_index = 0;
    while first_index < first.len() && second_index < second.len() {
        if first[first_index] < second[second_index] {
            result.push(first[first_index]);
            first_index += 1;
        } else {
            result.push(second[second_index]);
            second_index += 1;
        }
    }
    while first_index < first.len() {
        result.push(first[first_index]);
        first_index += 1;
    }
    while second_index < second.len() {
        result.push(second[second_index]);
        second_index += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combine_two_sorted_vectors() {
        assert_eq!(
            combine_two_sorted_vectors(vec![1, 2, 3], vec![4, 5, 6]),
            vec![1, 2, 3, 4, 5, 6]
        );
        assert_eq!(
            combine_two_sorted_vectors(vec![1, 3, 5], vec![2, 4, 6]),
            vec![1, 2, 3, 4, 5, 6]
        );
    }
}
