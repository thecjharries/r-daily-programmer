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

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func isJollyJumper(input []int) bool {
	diffs := make(map[int]struct{})
	for index := 1; index < len(input); index++ {
		diff := input[index] - input[index-1]
		if 0 > diff {
			diff = -diff
		}
		if len(input) <= diff {
			return false
		}
		diffs[diff] = struct{}{}
	}
	sortedDiffs := make([]int, 0, len(diffs))
	for diff := range diffs {
		sortedDiffs = append(sortedDiffs, diff)
	}
	sort.Ints(sortedDiffs)
	for index := 1; index < len(input); index++ {
		if index != sortedDiffs[index-1] {
			return false
		}
	}
	return true
}
