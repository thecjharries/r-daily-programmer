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
	"strings"
)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func findBaseUpTo16(input string) int {
	currentMax := int32(-1)
	for _, character := range input {
		if currentMax < character {
			currentMax = character
		}
	}
	base := strings.Index("0123456789abcdef", string(currentMax))
	if 0 < base {
		return base + 1
	}
	return 16
}

func findLowestBaseAndBase10(input string) string {
	base := findBaseUpTo16(input)
	representation, _ := strconv.ParseInt(input, base, 0)
	return fmt.Sprintf("base %d => %d", base, representation)
}
