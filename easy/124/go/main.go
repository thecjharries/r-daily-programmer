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

type Edge struct {
	Name   string
	Start  int
	Finish int
}

func (e *Edge) String() string {
	return e.Name
}

type GraphFromEdges []Edge

func (g *GraphFromEdges) Len() int {
	return len(*g)
}

func (g *GraphFromEdges) Less(i, j int) bool {
	return (*g)[i].Start <= (*g)[j].Start && (*g)[i].Finish < (*g)[j].Finish
}

func (g *GraphFromEdges) Swap(i, j int) {
	(*g)[i], (*g)[j] = (*g)[j], (*g)[i]
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}
