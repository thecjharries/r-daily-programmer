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

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn floyd_marshall(adjacency: Vec<Vec<bool>>) -> Vec<Vec<usize>> {
    let mut result = vec![vec![usize::MAX; adjacency.len()]; adjacency.len()];
    for (index, row) in adjacency.iter().enumerate() {
        for (inner_index, value) in row.iter().enumerate() {
            if index == inner_index {
                result[index][inner_index] = 0;
            } else if *value {
                result[index][inner_index] = 1;
            }
        }
    }
    for k in 0..adjacency.len() {
        for i in 0..adjacency.len() {
            for j in 0..adjacency.len() {
                if result[i][k] != usize::MAX && result[k][j] != usize::MAX {
                    result[i][j] = result[i][j].min(result[i][k] + result[k][j]);
                }
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
    fn floyd_marshall_computes_distance() {
        let input = vec![
            vec![false, true, false, true],
            vec![true, false, true, false],
            vec![false, true, false, true],
            vec![true, false, true, false],
        ];
        let output = vec![
            vec![0, 1, 2, 1],
            vec![1, 0, 1, 2],
            vec![2, 1, 0, 1],
            vec![1, 2, 1, 0],
        ];
        assert_eq!(output, floyd_marshall(input));
    }
}
