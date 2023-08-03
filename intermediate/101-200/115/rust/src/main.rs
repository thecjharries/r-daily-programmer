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

use std::collections::BTreeMap;

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn find_sum_pairs(target: i32, numbers: Vec<i32>) -> Vec<(i32, i32)> {
    let mut pairs = Vec::<(i32, i32)>::new();
    let mut numbers = BTreeMap::from_iter(numbers.into_iter().map(|number| (number, true)));
    for number in numbers.keys() {
        let difference = target - number;
        if numbers.contains_key(&difference) {
            pairs.push((*number, difference));
        }
    }
    pairs
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_sum_pairs() {
        assert_eq!(vec![(1, 4), (4, 1)], find_sum_pairs(5, vec![1, -3, 4, 10]));
        assert_eq!(
            vec![(1, 6), (3, 4), (4, 3), (6, 1)],
            find_sum_pairs(7, vec![10, -8, 2, 1, 4, -9, 6, 1, 9, -10, -5, 2, 3, 7])
        );
    }
}
