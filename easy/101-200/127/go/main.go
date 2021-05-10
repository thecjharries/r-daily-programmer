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

var zPrint = fmt.Println

func main() {
	promptMcCarthy91(99, false)
}

func promptMcCarthy91(start int, isRecursive bool) int {
	if !isRecursive {
		_, _ = zPrint(fmt.Sprintf("M(%d)", start))
	}
	if 100 < start {
		formatString := "M(%d) since %d is greater than 100"
		if 101 == start {
			formatString = "%d since %d is greater than 100"
		}
		_, _ = zPrint(fmt.Sprintf(formatString, start-10, start))
		return start - 10
	}
	_, _ = zPrint(fmt.Sprintf("M(M(%d)) since %d is equal to or less than 100", start+11, start))
	return promptMcCarthy91(promptMcCarthy91(start+11, true), true)
}
