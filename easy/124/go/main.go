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
	"fmt"
	"sort"
)

type Edge struct {
	Name   string
	Start  int
	Finish int
}

func (e *Edge) String() string {
	return e.Name
}

type GraphFromEdges []Edge

func (g GraphFromEdges) Len() int {
	return len(g)
}

func (g GraphFromEdges) Less(i, j int) bool {
	return g[i].Start <= g[j].Start && g[i].Finish < g[j].Finish
}

func (g GraphFromEdges) Swap(i, j int) {
	g[i], g[j] = g[j], g[i]
}

func (g GraphFromEdges) String() (output string) {
	for _, edge := range g {
		output += fmt.Sprintf("%s ", edge.String())
	}
	return
}

var zPrint = fmt.Println

func main() {
	graph := GraphFromEdges{
		{"F", 2, 3},
		{"B", 1, 2},
		{"D", 6, 5},
		{"C", 6, 7},
		{"E", 5, 4},
		{"A", 3, 4},
	}
	sort.Sort(graph)
	_, _ = zPrint("hello world")
}
