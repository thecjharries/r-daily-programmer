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

fn build_directed_adjacency_matrix(vertex_count: usize, input: &str) -> Vec<Vec<usize>> {
    let mut matrix = vec![vec![0; vertex_count]; vertex_count];
    for line in input.split('\n') {
        let line = line.trim();
        let mut line_iter = line.split(" -> ");
        let source = line_iter.next().unwrap().parse::<usize>().unwrap();
        for destination in line_iter.next().unwrap().split(" ") {
            matrix[source][destination.parse::<usize>().unwrap()] += 1;
        }
    }
    matrix
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_directed_adjacency_matrix_creates_proper_matrix() {
        let input = "0 -> 1
        1 -> 2
        2 -> 4
        3 -> 4
        0 -> 3";
        let result = vec![
            vec![0, 1, 0, 1, 0],
            vec![0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 1],
            vec![0, 0, 0, 0, 1],
            vec![0, 0, 0, 0, 0],
        ];
        assert_eq!(result, build_directed_adjacency_matrix(5, input));
    }
}
