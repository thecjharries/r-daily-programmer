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

fn find_majority_index(input: Vec<u32>) -> i32 {
    let sum = input.iter().sum::<u32>();
    let mut necessary_majority = sum / 2;
    if 0 == sum % 2 {
        necessary_majority += 1;
    }
    let mut majority_index: i32 = -1;
    for (index, value) in input.iter().enumerate() {
        if &necessary_majority < value {
            if -1 == majority_index {
                majority_index = index as i32;
            } else {
                if &input[majority_index as usize] < value {
                    majority_index = index as i32;
                }
            }
        }
    }
    majority_index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_majority_index() {
        assert_eq!(find_majority_index(vec![1, 2, 4]), 2);
        assert_eq!(find_majority_index(vec![1, 1, 1, 1, 1]), -1);
    }
}
