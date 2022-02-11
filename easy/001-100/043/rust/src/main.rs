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
    pub fn new(value: i32, depth: i32, parent: Option<BinaryNode>) -> BinaryNode {
        BinaryNode {
            parent,
            left: None,
            right: None,
            value,
            depth,
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

    pub fn lowest_common_ancestor(first: BinaryNode, second: BinaryNode) {

    }
}

fn main() {
    println!("rad");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_tree_lowest_common_ancestor() {
        let mut tree = BinaryTree::new();
        let root = BinaryNode::new(1, 0, None);
        let left = BinaryNode::new(2, 1, Some(root));
        let right = BinaryNode::new(3, 1, Some(root));
        let left_left = BinaryNode::new(4, 2, Some(left));
        let left_right = BinaryNode::new(5, 2, Some(left));
        let right_left = BinaryNode::new(6, 2, Some(right));
        let right_right = BinaryNode::new(7, 2, Some(right));
        tree.root = Some(Box::new(root));
        tree.root.as_mut().unwrap().left = Some(Box::new(left));
        tree.root.as_mut().unwrap().right = Some(Box::new(right));
        tree.root.as_mut().unwrap().left.as_mut().unwrap().left = Some(Box::new(left_left));
        tree.root.as_mut().unwrap().left.as_mut().unwrap().right = Some(Box::new(left_right));
        tree.root.as_mut().unwrap().right.as_mut().unwrap().left = Some(Box::new(right_left));
        tree.root.as_mut().unwrap().right.as_mut().unwrap().right = Some(Box::new(right_right));

    }
}
