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

type TimeRange [2]int

type TimeRanges []TimeRange

func (t TimeRanges) Len() int {
	return len(t)
}

func (t TimeRanges) Less(i, j int) bool {
	return t[i][0] < t[j][0] || (t[i][0] == t[j][0] && t[i][1] < t[j][1])
}

func (t TimeRanges) Swap(i, j int) {
	t[i], t[j] = t[j], t[i]
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func determineLightLength(lights []TimeRange) int {
	sortedLights := make(TimeRanges, len(lights))
	copy(sortedLights, lights)
	sort.Sort(sortedLights)
	min, max, total := -1, -1, 0
	for _, timeRange := range sortedLights {
		if max < timeRange[0] {
			total += max - min
			min = timeRange[0]
			max = timeRange[1]
		}
		if timeRange[0] <= max && timeRange[1] > max {
			max = timeRange[1]
		}
	}
	total += max - min
	return total
}
