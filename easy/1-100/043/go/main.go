// Copyright 2021 CJ Harries
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

package main

import "fmt"

type BinaryNode struct {
	Parent *BinaryNode
	Left   *BinaryNode
	Right  *BinaryNode
	Depth  int
	Value  interface{}
}

func (n *BinaryNode) NewChild(value interface{}) *BinaryNode {
	return &BinaryNode{
		Parent: n,
		Depth:  n.Depth + 1,
		Value:  value,
	}
}

type BinaryTree struct {
	Root *BinaryNode
}

func (t *BinaryTree) LowestCommonAncestor(first, second *BinaryNode) *BinaryNode {
	if first.Depth < second.Depth {
		return t.LowestCommonAncestor(first, second.Parent)
	} else if first.Depth > second.Depth {
		return t.LowestCommonAncestor(first.Parent, second)
	}
	if first.Parent == second.Parent {
		return first.Parent
	}
	return t.LowestCommonAncestor(first.Parent, second.Parent)
}

func main() {
	fmt.Println("hello world")
}
