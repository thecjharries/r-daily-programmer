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

use std::cmp::min;

fn main() {
    println!("rad");
}

fn reverse_by_block_size(input: Vec<u32>, block_size: usize) -> Vec<u32> {
    let mut output = Vec::new();
    for right_index in (0..input.len()).step_by(block_size) {
        for left_index in (right_index..min(right_index + block_size, input.len())).rev() {
            output.push(input[left_index]);
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_by_block_size() {
        assert_eq!(reverse_by_block_size(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 3), vec![3, 2, 1, 6, 5, 4, 9, 8, 7, 10]);
        assert_eq!(reverse_by_block_size(vec![12, 24, 32, 44, 55, 66], 2), vec![24, 12, 44, 32, 66, 55])
    }
}
