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

struct BinaryNode {
    parent: Option<Box<BinaryNode>>,
    left: Option<Box<BinaryNode>>,
    right: Option<Box<BinaryNode>>,
    value: i32,
    depth: i32,
}

impl BinaryNode {
    pub fn new(value: i32, parent: Option<BinaryNode>) -> BinaryNode {
        BinaryNode {
            parent,
            left: None,
            right: None,
            value,
            depth: 0,
        }
    }
}

struct BinaryTree {
    root: Option<Box<BinaryNode>>,
}

impl BinaryTree {
    pub fn new() -> Self {
        BinaryTree { root: None }
    }
}

fn main() {
    println!("rad");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub() {
        assert_eq!(2 + 2, 4);
    }
}
