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

var promptTagMap = map[string]string{
	"a": "bc",
	"b": "a",
	"c": "aaa",
}

var zPrint = fmt.Println

func main() {
	_, _ = zPrint("hello world")
}

func processCollatzTag(input string, tagMap map[string]string) (result []string) {
	current := input
	for 1 < len(current) {
		next, _ := tagMap[current[0:1]]
		current = current[2:] + next
		result = append(result, current)
	}
	return
}
