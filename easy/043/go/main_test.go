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

import (
	"testing"

	. "gopkg.in/check.v1"
)

func TestRootMain(t *testing.T) { TestingT(t) }

type MainSuite struct {}

var _ = Suite(&MainSuite{})

func (s *MainSuite) TestMain(c *C) {

}

func (s *MainSuite) TestBinaryNodeNewChild(c *C) {
	parent := &BinaryNode{Depth: 0, Value: 0}
	parent.Left = parent.NewChild("left")
	c.Assert(parent.Left.Depth, Equals, 1)
	c.Assert(parent.Left.Parent, Equals, parent)
}

func (s *MainSuite) TestBinaryTreeLowestCommonAncestorLowerFirst(c *C) {
	root := &BinaryNode{Depth: 0, Value: 0}
	root.Left = root.NewChild(1)
	root.Left.Left = root.Left.NewChild(2)
	root.Right = root.NewChild(1)
	tree := &BinaryTree{Root: root}
	c.Assert(tree.LowestCommonAncestor(root.Left.Left, root.Right), Equals, root)
}

func (s *MainSuite) TestBinaryTreeLowestCommonAncestorLowerSecond(c *C) {
	root := &BinaryNode{Depth: 0, Value: 0}
	root.Left = root.NewChild(1)
	root.Left.Left = root.Left.NewChild(2)
	root.Right = root.NewChild(1)
	tree := &BinaryTree{Root: root}
	c.Assert(tree.LowestCommonAncestor(root.Right, root.Left.Left), Equals, root)
}
