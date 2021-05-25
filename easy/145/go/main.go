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
	"strings"
)

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func generateTree(baseWidth int, leaves, trunk string) (output string) {
	if 0 == baseWidth%2 {
		baseWidth += 1
	}
	for leafCount := 1; leafCount <= baseWidth; leafCount += 2 {
		output += fmt.Sprintf(
			"%s%s%s",
			strings.Repeat(" ", (baseWidth-leafCount)/2),
			strings.Repeat(leaves, leafCount),
			strings.Repeat(" ", (baseWidth-leafCount)/2),
		)
	}
	output += fmt.Sprintf(
		"%s%s%s",
		strings.Repeat(" ", (baseWidth-3)/2),
		strings.Repeat(leaves, 3),
		strings.Repeat(" ", (baseWidth-3)/2),
	)
	return
}
