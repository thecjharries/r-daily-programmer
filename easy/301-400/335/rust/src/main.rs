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

use std::collections::HashMap;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn calculate_consecutive_distance_rating(input: Vec<u32>) -> u32 {
    let mut sorted = input.clone();
    sorted.sort();
    let values: HashMap<&u32, u32> = HashMap::from_iter(
        input
            .iter()
            .enumerate()
            .map(|(index, value)| (value, index as u32)),
    );
    let mut rating: u32 = 0;
    for index in 1..sorted.len() {
        if 1 == sorted[index] - sorted[index - 1] {
            let first_index = *values.get(&sorted[index - 1]).unwrap();
            let second_index = *values.get(&sorted[index]).unwrap();
            if first_index < second_index {
                rating += second_index - first_index;
            } else {
                rating += first_index - second_index;
            }
        }
    }
    rating
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(
            26,
            calculate_consecutive_distance_rating(vec![31, 63, 53, 56, 96, 62, 73, 25, 54, 55, 64])
        );
        assert_eq!(
            20,
            calculate_consecutive_distance_rating(vec![77, 39, 35, 38, 41, 42, 76, 73, 40, 31, 10])
        );
        assert_eq!(
            15,
            calculate_consecutive_distance_rating(vec![30, 63, 57, 87, 37, 31, 58, 83, 34, 76, 38])
        );
        assert_eq!(
            3,
            calculate_consecutive_distance_rating(vec![18, 62, 55, 92, 88, 57, 90, 10, 11, 96, 12])
        );
        assert_eq!(
            6,
            calculate_consecutive_distance_rating(vec![26, 8, 7, 25, 52, 17, 45, 64, 11, 35, 12])
        );
        assert_eq!(
            13,
            calculate_consecutive_distance_rating(vec![
                89, 57, 21, 55, 56, 81, 54, 100, 22, 62, 50
            ])
        );
    }
}
