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
	"strconv"
)

type IntConcatenation []int

func (i IntConcatenation) Len() int {
	return len(i)
}

func (i IntConcatenation) Swap(j, k int) {
	i[j], i[k] = i[k], i[j]
}

func (i IntConcatenation) Less(j, k int) bool {
	first, _ := strconv.Atoi(strconv.Itoa(i[j]) + strconv.Itoa(i[k]))
	second, _ := strconv.Atoi(strconv.Itoa(i[k]) + strconv.Itoa(i[j]))
	return first < second
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func findLargestConcatenation(input []int) (output int) {
	return
}
