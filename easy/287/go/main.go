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
	"strconv"
	"strings"
)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func sortDigits(input int, ascending bool) (output int) {
	inputAsString := strconv.Itoa(input)
	if 4 > len(inputAsString) {
		inputAsString = strings.Repeat("0", 4-len(inputAsString)) + inputAsString
	}
	exploded := strings.Split(inputAsString, "")
	if ascending {
		sort.Strings(exploded)
	} else {
		sort.Sort(sort.Reverse(sort.StringSlice(exploded)))
	}
	output, _ = strconv.Atoi(strings.Join(exploded, ""))
	return
}

func kaprekarIterationCount(input int) (iterations int) {
	return
}
