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

#[derive(Debug)]
struct DirectedGraph {
    edges: BTreeMap<u32, Vec<u32>>,
}

impl DirectedGraph {
    fn new(graph_edges: Vec<(u32, u32)>) -> Self {
        let edges = graph_edges
            .iter()
            .fold(BTreeMap::new(), |mut acc, (from, to)| {
                acc.entry(*from).or_insert_with(Vec::new).push(*to);
                acc
            });
        Self { edges }
    }

    fn find_a_cycle(&self, current_cycle: Vec<u32>) -> Option<Vec<u32>> {
        let last_node = *current_cycle.last().unwrap();
        if let Some(next_nodes) = self.edges.get(&last_node) {
            for next_node in next_nodes {
                if current_cycle.contains(next_node) {
                    return Some(current_cycle);
                }
                let mut new_cycle = current_cycle.clone();
                new_cycle.push(*next_node);
                if let Some(cycle) = self.find_a_cycle(new_cycle) {
                    return Some(cycle);
                }
            }
        }
        None
    }
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directedgraph_new_builds_edge_map() {
        let graph = DirectedGraph::new(vec![(1, 2), (2, 3), (3, 1), (3, 4)]);
        assert_eq!(vec![2], graph.edges[&1]);
        assert_eq!(vec![1, 4], graph.edges[&3]);
    }

    #[test]
    fn directedgraph_find_a_cycle_finds_cycle() {
        let graph = DirectedGraph::new(vec![(1, 2), (2, 3), (3, 1), (3, 4)]);
        assert_eq!(vec![1, 2, 3], graph.find_a_cycle(vec![1]).unwrap());
        let graph = DirectedGraph::new(vec![(1, 2), (2, 3), (3, 4)]);
        assert_eq!(None, graph.find_a_cycle(vec![1]));
    }
}
