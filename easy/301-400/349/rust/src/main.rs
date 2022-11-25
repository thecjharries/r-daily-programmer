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

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn determine_change(needed: u32, available: Vec<u32>, max_coins: usize) -> Vec<u32> {
    let mut result = Vec::new();
    for size in 1..=max_coins {
        for combination in available.iter().combinations(size) {
            if combination.iter().map(|&x| *x).sum::<u32>() == needed {
                result = combination.iter().map(|&x| *x).collect();
                break;
            }
        }
    }
    result
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(vec![5, 5], determine_change(10, vec![5, 5, 2, 2, 1], 3));
        assert_eq!(
            vec![] as Vec<u32>,
            determine_change(10, vec![5, 5, 2, 2, 1], 1)
        );
    }
}
