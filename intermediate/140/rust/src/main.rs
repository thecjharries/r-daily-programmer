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

fn parse_adjacency_matrix(input: &str) -> Vec<Vec<bool>> {
    let binding = input.trim();
    binding
        .split('\n')
        .map(|line| {
            let binding = line.trim();
            binding
                .split_whitespace()
                .map(|character| character == "1")
                .collect()
        })
        .collect()
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

fn find_graph_radius(adjacency: Vec<Vec<bool>>) -> usize {
    floyd_marshall(adjacency)
        .iter()
        .map(|row| row.iter().max().unwrap())
        .min()
        .unwrap()
        .clone()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_adjacency_matrix() {
        let input = "0 1 0 1\n1 0 1 0\n0 1 0 1\n1 0 1 0\n";
        let output = vec![
            vec![false, true, false, true],
            vec![true, false, true, false],
            vec![false, true, false, true],
            vec![true, false, true, false],
        ];
        assert_eq!(output, parse_adjacency_matrix(input));
    }

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

    #[test]
    fn find_graph_radius_finds_radius() {
        let input = vec![
            vec![false, true, false, true],
            vec![true, false, true, false],
            vec![false, true, false, true],
            vec![true, false, true, false],
        ];
        assert_eq!(2, find_graph_radius(input));
        // prompt graph
        let input = parse_adjacency_matrix(
            "0 1 0 0 1 1 0 0 0 0
        1 0 1 0 0 0 1 0 0 0
        0 1 0 1 0 0 0 1 0 0
        0 0 1 0 1 0 0 0 1 0
        1 0 0 1 0 0 0 0 0 1
        1 0 0 0 0 0 0 1 1 0
        0 1 0 0 0 0 0 0 1 1
        0 0 1 0 0 1 0 0 0 1
        0 0 0 1 0 1 1 0 0 0
        0 0 0 0 1 0 1 1 0 0",
        );
        assert_eq!(2, find_graph_radius(input));
        // Nauru graph
        let input = parse_adjacency_matrix(
            "0 1 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 0
        1 0 0 1 0 0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0 0
        0 0 0 1 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1
        0 1 1 0 0 0 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0
        0 0 1 0 0 1 0 0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 0 0
        1 0 0 0 1 0 0 0 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 1 0 0 0 1 0 0 0 0 0 0 0 1 0 0 0 0
        0 0 0 0 0 0 1 0 0 1 0 0 0 1 0 0 0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0 0 1 1 0 0 0 0 0 0 0 0 0 0 0 1 0
        0 0 0 1 0 0 0 1 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0 1 0 0 1 0 0 0 0 1 0 0 0 0 0 0 0
        0 0 0 0 0 1 1 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 0 0 1 1 0 0 0 0 0
        0 0 0 0 0 0 0 1 0 0 0 0 1 0 0 1 0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 1 0 0 0 1 0 0 0
        0 1 0 0 0 0 0 0 0 0 0 0 0 1 1 0 0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0 0 0 1 0 0 0 1 0 0 1 0 0 0 0 0 0
        0 0 0 0 1 0 0 0 0 0 0 0 1 0 0 0 1 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 0 0 1 0 0 0 1
        0 0 0 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 1 0 0 1 0 0
        0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 0 0 1 1 0
        1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 1 0 0 0
        0 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 1 0 0 1
        0 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 0 0 1 0",
        );
        assert_eq!(4, find_graph_radius(input));
    }
}
