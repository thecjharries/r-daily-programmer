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

func transposeText(input string) string {
	exploded := strings.Split(input, "\n")
	var outputExploded []string
	for lineIndex, line := range exploded {
		for characterIndex, _ := range line {
			if len(outputExploded) < characterIndex+1 {
				outputExploded = append(outputExploded, "")
			}
			if len(outputExploded[characterIndex]) < lineIndex {
				fmt.Println(line, characterIndex, line[characterIndex:characterIndex+1])
				outputExploded[characterIndex] += strings.Repeat(" ", lineIndex-len(outputExploded[characterIndex]))
			}
			outputExploded[characterIndex] = strings.TrimRight(fmt.Sprintf("%s%s", outputExploded[characterIndex], line[characterIndex:characterIndex+1]), " ")
		}
	}
	return strings.Join(outputExploded, "\n")
}
