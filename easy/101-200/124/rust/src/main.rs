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

#[derive(Debug, PartialEq)]
struct Edge {
    first_node: u32,
    second_node: u32,
}

fn build_edge(first_node: u32, second_node: u32) -> Edge {
    if first_node < second_node {
        Edge {
            first_node,
            second_node,
        }
    } else {
        Edge {
            first_node: second_node,
            second_node: first_node,
        }
    }
}

fn main() {
    println!("rad");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_edge() {
        assert_eq!(
            build_edge(1, 2),
            Edge {
                first_node: 1,
                second_node: 2
            }
        );
        assert_eq!(
            build_edge(2, 1),
            Edge {
                first_node: 1,
                second_node: 2
            }
        );
    }

    #[test]
    fn test_stub() {
        assert_eq!(2 + 2, 4);
    }
}
