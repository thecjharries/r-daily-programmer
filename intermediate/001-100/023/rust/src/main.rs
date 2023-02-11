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

use std::collections::HashSet;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn generate_non_mcnugget_numbers() -> Vec<u32> {
    let threes = (6..44).step_by(3).collect::<HashSet<u32>>();
    let twenties = (20..44).step_by(20).collect::<HashSet<u32>>();
    let offset = (26..44).step_by(3).collect::<HashSet<u32>>();
    let complete = (1..44).collect::<HashSet<u32>>();
    let mut result: Vec<u32> = complete
        .difference(&threes)
        .map(|&value| value.clone())
        .collect::<HashSet<u32>>()
        .difference(&twenties)
        .map(|&value| value.clone())
        .collect::<HashSet<u32>>()
        .difference(&offset)
        .map(|&value| value.clone())
        .collect();
    result.sort();
    result
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_non_mcnugget_numbers() {
        assert_eq!(
            vec![1, 2, 3, 4, 5, 7, 8, 10, 11, 13, 14, 16, 17, 19, 22, 23, 25, 28, 31, 34, 37, 43],
            generate_non_mcnugget_numbers()
        );
    }
}
