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

fn combine_sorted(unsorted: &[i32], sorted_with_space: &mut [i32]) {
    for unsorted_index in 0..unsorted.len() {
        for sorted_with_space_index in 1..sorted_with_space.len() {
            if 0 != sorted_with_space[sorted_with_space_index] {
                if sorted_with_space[sorted_with_space_index] < unsorted[unsorted_index] {
                    sorted_with_space[sorted_with_space_index - 1] =
                        sorted_with_space[sorted_with_space_index];
                } else {
                    sorted_with_space[sorted_with_space_index - 1] = unsorted[unsorted_index];
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combine_sorted() {
        let unsorted = vec![692, 1, 32];
        let mut sorted_with_space = vec![0, 0, 0, 14, 15, 123, 2431];
        combine_sorted(&unsorted, &mut sorted_with_space);
        assert_eq!(sorted_with_space, vec![1, 14, 15, 32, 123, 692, 2431]);
    }
}
