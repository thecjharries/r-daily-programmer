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

fn check_ullman(numbers: Vec<f64>, target: f64, count: usize) -> Vec<f64> {
    for possible_set in numbers.into_iter().combinations(count) {
        let sum = possible_set.iter().sum::<f64>();
        if sum <= target {
            return possible_set;
        }
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_ullman() {
        assert_eq!(
            check_ullman(
                vec![
                    18.1, 55.1, 91.2, 74.6, 73.0, 85.9, 73.9, 81.4, 87.1, 49.3, 88.8, 5.7, 26.3,
                    7.1, 58.2, 31.7, 5.8, 76.9, 16.5, 8.1, 48.3, 6.8, 92.4, 83.0, 19.6
                ],
                98.2,
                3
            ),
            vec![31.7, 16.5, 19.6]
        );
        assert_eq!(
            check_ullman(
                vec![
                    18.1, 55.1, 91.2, 74.6, 73.0, 85.9, 73.9, 81.4, 87.1, 49.3, 88.8, 5.7, 26.3,
                    7.1, 58.2, 31.7, 5.8, 76.9, 16.5, 8.1, 48.3, 6.8, 92.4, 83.0, 19.6
                ],
                1,
                3
            ),
            vec![]
        );
    }
}
