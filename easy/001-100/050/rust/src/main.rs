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

fn find_indices_that_sum_to_target(target: i32, numbers: &Vec<i32>) -> Vec<usize> {
    for (first_index, first_value) in numbers.iter().enumerate() {
        for (second_index, second_value) in numbers[first_index+1..].iter().enumerate() {
            if first_value + second_value == target {
                return vec![first_index, first_index+1+second_index];
            }
        }
    }
    return Vec::new();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_indices_that_sum_to_target() {
        assert_eq!(find_indices_that_sum_to_target(100, &vec![5, 75, 25]), vec![1,2]);
        assert_eq!(find_indices_that_sum_to_target(200, &vec![150, 24, 79, 50, 88, 345, 3]), vec![0, 3]);
        assert_eq!(find_indices_that_sum_to_target(8, &vec![2, 1, 9, 4, 4, 56, 90, 3]), vec![3, 4]);
        assert_eq!(find_indices_that_sum_to_target(100, &vec![1, 1, 1, 1]), vec![]);
    }
}
