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

use itertools::Itertools;

fn main() {
    println!("rad");
}

fn generate_combinations_of_size(input: Vec<i32>, size: usize) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    for combination in input.into_iter().combinations(size) {
        result.push(combination.to_vec());
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(
            generate_combinations_of_size(vec![1, 2, 3], 2),
            vec![vec![1, 2], vec![1, 3], vec![2, 3]]
        );
        assert_eq!(
            generate_combinations_of_size(vec![1, 2, 3, 4, 5], 3),
            vec![
                vec![1, 2, 3],
                vec![1, 2, 4],
                vec![1, 2, 5],
                vec![1, 3, 4],
                vec![1, 3, 5],
                vec![1, 4, 5],
                vec![2, 3, 4],
                vec![2, 3, 5],
                vec![2, 4, 5],
                vec![3, 4, 5]
            ]
        );
    }
}
