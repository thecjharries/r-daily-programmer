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

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn find_zero_sum_triplets(input: Vec<i32>) -> Vec<Vec<i32>> {
    todo!();
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_zero_sum_triplets() {
        assert_eq!(
            vec![
                vec![-5, -4, 9],
                vec![-6, 3, 3],
                vec![-5, 1, 4],
                vec![-4, -4, 8],
                vec![-9, 1, 8],
                vec![-4, 1, 3],
                vec![-8, 1, 7],
                vec![-8, 4, 4]
            ],
            find_zero_sum_triplets(vec![
                9, -6, -5, 9, 8, 3, -4, 8, 1, 7, -4, 9, -9, 1, 9, -9, 9, 4, -6, -8
            ])
        );
        assert_eq!(
            vec![
                vec![-3, -1, 4],
                vec![-2, -2, 4],
                vec![-5, 1, 4],
                vec![-3, -2, 5],
                vec![-7, 2, 5],
                vec![-1, -1, 2],
                vec![-3, 1, 2]
            ],
            find_zero_sum_triplets(vec![4, 5, -1, -2, -7, 2, -5, -3, -7, -3, 1])
        );
        assert_eq!(
            vec![
                vec![-1, -1, 2],
                vec![-6, 1, 5],
                vec![-3, 1, 2],
                vec![-7, 2, 5]
            ],
            find_zero_sum_triplets(vec![-1, -6, -3, -7, 5, -8, 2, -8, 1])
        );
        assert_eq!(
            vec![vec![-5, -4, 9], vec![-1, -1, 2], vec![-4, 2, 2]],
            find_zero_sum_triplets(vec![-5, -1, -4, 2, 9, -9, -6, -1, -7])
        );
    }
}
