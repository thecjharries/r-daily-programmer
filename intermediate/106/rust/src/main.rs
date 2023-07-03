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

use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct JugState {
    first: u32,
    second: u32,
}

#[cfg(not(tarpaulin_include))]
fn main() {
    println!("rad");
}

fn jug_solver(first: u32, second: u32, desired: u32) -> Vec<JugState> {
    let mut steps: usize = 0;
    let mut visited: HashSet<JugState> = HashSet::new();
    let mut costs: HashMap<JugState, usize> = HashMap::new();
    let mut previous: HashMap<JugState, JugState> = HashMap::new();
    let _val = costs
        .entry(JugState {
            first: 0,
            second: 0,
        })
        .or_insert(0);
    let mut queue = vec![JugState {
        first: 0,
        second: 0,
    }];
    let mut tail = JugState {
        first: u32::MAX,
        second: u32::MAX,
    };
    let mut best_cost = usize::MAX;
    loop {
        if queue.is_empty() {
            break;
        }
        let mut current = queue.pop().unwrap();
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current.clone());
        if (current.first == desired || current.second == desired) && costs[&current] < best_cost {
            tail = current.clone();
            best_cost = costs[&current];
        }
        let (m, n) = (current.first, current.second);
        let mut neighbors = vec![
            JugState {
                first: first,
                second: n,
            },
            JugState {
                first: m,
                second: second,
            },
            JugState {
                first: 0,
                second: n,
            },
            JugState {
                first: m,
                second: 0,
            },
        ];
        if m + n < first {
            neighbors.push(JugState {
                first: m + n,
                second: 0,
            });
        } else {
            neighbors.push(JugState {
                first: first,
                second: m + n - first,
            });
        }
        if m + n < second {
            neighbors.push(JugState {
                first: 0,
                second: m + n,
            });
        } else {
            neighbors.push(JugState {
                first: m + n - second,
                second: second,
            });
        }
        let cost = costs[&current] + 1;
        for neighbor in neighbors {
            if !visited.contains(&neighbor) {
                if !costs.contains_key(&neighbor) || cost < costs[&neighbor] {
                    costs.insert(neighbor.clone(), cost);
                    previous.insert(neighbor.clone(), current.clone());
                }
                queue.push(neighbor);
            }
        }
    }
    if tail.first == u32::MAX {
        return vec![];
    }
    let mut path = vec![tail.clone()];
    while let Some(previous_state) = previous.get(&tail) {
        path.push(previous_state.clone());
        tail = previous_state.clone();
    }
    path.reverse();
    path
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jug_solver() {
        assert_eq!(9, jug_solver(3, 5, 4).len());
    }
}
