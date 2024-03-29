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

fn create_adjacency_matrix(vertex_count: usize, edges: Vec<(usize, usize)>) -> Vec<Vec<usize>> {
    let mut adjacency_matrix = vec![vec![0; vertex_count]; vertex_count];
    for (first_vertex, second_vertex) in edges {
        adjacency_matrix[first_vertex - 1][second_vertex - 1] += 1;
        adjacency_matrix[second_vertex - 1][first_vertex - 1] += 1;
    }
    adjacency_matrix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(
            vec![vec![0, 1, 1], vec![1, 0, 0], vec![1, 0, 0]],
            create_adjacency_matrix(3, vec![(1, 2), (1, 3)])
        );
    }
}
